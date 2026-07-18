#![cfg(all(
    feature = "blas-level1-concurrency-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Reviewed-source-backend concurrency qualification. The counters surround
//! the exact FFI call, so `maximum_active > 1` is native-entry evidence rather
//! than merely concurrent Rust thread activity.

use std::sync::{Arc, Barrier};
use std::time::Duration;

use slatec::blas::level1::*;
use slatec::blas1_concurrency_test_support::{reset, snapshot};
use slatec::native_serialization_test_support as hosted;
use slatec::ode::{OdeOptions, OdeSession, OdeTolerance, OdeTolerances};

const LARGE: usize = 1_000_000;

#[test]
fn same_routine_calls_enter_native_code_simultaneously() {
    reset();
    let barrier = Arc::new(Barrier::new(9));
    let threads = (0..8)
        .map(|thread| {
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                let mut values = vec![thread as f64 + 1.0; LARGE];
                barrier.wait();
                for _ in 0..8 {
                    dscal(1.000_001, &mut values).unwrap();
                }
                let expected = (thread as f64 + 1.0) * 1.000_001_f64.powi(8);
                assert!((values[LARGE / 2] - expected).abs() < 1.0e-10);
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    for thread in threads {
        thread.join().unwrap();
    }
    let observed = snapshot();
    eprintln!("same-routine native overlap: {observed:?}");
    assert_eq!(observed.active, 0);
    assert!(
        observed.maximum_active > 1,
        "candidate FFI entries never overlapped: {observed:?}"
    );
}

#[test]
fn mixed_precision_contiguous_and_strided_calls_are_independent() {
    reset();
    let barrier = Arc::new(Barrier::new(9));
    let threads = (0..8)
        .map(|thread| {
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                for round in 0..12 {
                    if (thread + round) % 2 == 0 {
                        let input = deterministic_f64(250_000, thread as u64 + round as u64);
                        let expected = input.iter().map(|value| value.abs()).sum::<f64>();
                        let actual = if round % 3 == 0 {
                            dasum_strided(input.len(), &input, -1).unwrap()
                        } else {
                            dasum(&input).unwrap()
                        };
                        assert!((actual - expected).abs() < 1.0e-7 * expected.max(1.0));
                    } else {
                        let input = deterministic_f32(250_000, thread as u64 + round as u64);
                        let mut output = vec![0.0_f32; input.len()];
                        if round % 3 == 0 {
                            scopy_strided(input.len(), &input, -1, &mut output, -1).unwrap();
                        } else {
                            scopy(&input, &mut output).unwrap();
                        }
                        assert_eq!(output, input);
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    for thread in threads {
        thread.join().unwrap();
    }
    let observed = snapshot();
    eprintln!("mixed-routine native overlap: {observed:?}");
    assert_eq!(observed.active, 0);
    assert!(observed.maximum_active > 1);

    let mut empty = [];
    let before = snapshot().maximum_active;
    daxpy(2.0, &[], &mut empty).unwrap();
    assert_eq!(snapshot().maximum_active, before, "zero length reached FFI");
}

#[test]
fn strided_canaries_and_repeated_thread_creation_remain_intact() {
    reset();
    for generation in 0..20 {
        let threads = (0..4)
            .map(|thread| {
                std::thread::spawn(move || {
                    let mut values = vec![-777.0_f64; 20_001];
                    for index in (0..values.len()).step_by(2) {
                        values[index] = (generation * 4 + thread + 1) as f64;
                    }
                    dscal_strided(10_001, 2.0, &mut values, -2).unwrap();
                    assert!(
                        values
                            .iter()
                            .skip(1)
                            .step_by(2)
                            .all(|value| *value == -777.0)
                    );
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
    }
    assert_eq!(snapshot().active, 0);
}

#[test]
fn qualified_candidate_can_overlap_exclusive_solver_only_on_reviewed_source_profile() {
    reset();
    hosted::reset();
    let barrier = Arc::new(Barrier::new(3));
    let solver_barrier = Arc::clone(&barrier);
    let solver = std::thread::spawn(move || {
        let mut session = OdeSession::new(
            0.0_f64,
            vec![1.0],
            |_time, state, derivative| -> Result<(), ()> {
                std::thread::sleep(Duration::from_millis(2));
                derivative[0] = -state[0];
                Ok(())
            },
            OdeTolerances {
                relative: 1.0e-9,
                absolute: OdeTolerance::Scalar(1.0e-11),
            },
            OdeOptions::default(),
        )
        .unwrap();
        solver_barrier.wait();
        session.integrate_to(1.0).unwrap();
    });
    let blas_barrier = Arc::clone(&barrier);
    let blas = std::thread::spawn(move || {
        let mut values = vec![1.0_f64; 100_000];
        blas_barrier.wait();
        for _ in 0..2_000 {
            dscal(1.0, &mut values).unwrap();
        }
    });
    barrier.wait();
    solver.join().unwrap();
    blas.join().unwrap();

    let candidate = snapshot();
    let exclusive = hosted::snapshot();
    eprintln!("candidate/exclusive overlap: candidate={candidate:?} hosted={exclusive:?}");
    assert!(
        candidate.hosted_overlaps > 0,
        "no candidate/solver overlap observed"
    );
    assert_eq!(exclusive.active, 0);
    assert_eq!(exclusive.maximum_active, 1);
}

fn deterministic_f64(length: usize, seed: u64) -> Vec<f64> {
    deterministic_bits(length, seed)
        .into_iter()
        .map(|bits| (bits as i64 as f64) / i64::MAX as f64)
        .collect()
}

fn deterministic_f32(length: usize, seed: u64) -> Vec<f32> {
    deterministic_bits(length, seed)
        .into_iter()
        .map(|bits| (bits as i32 as f32) / i32::MAX as f32)
        .collect()
}

fn deterministic_bits(length: usize, seed: u64) -> Vec<u64> {
    let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    (0..length)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            state
        })
        .collect()
}
