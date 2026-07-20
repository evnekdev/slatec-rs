//! Generated Batch B canonical-path compile probe.
//! Regenerate with `slatec-corpus generate-raw-batch-b --offline`.

#[cfg(feature = "all")]
mod paths {
    #[allow(unused_imports)]
    use slatec_sys::{
        linear_algebra::sparse::callbacks::dbcg, linear_algebra::sparse::callbacks::dcg,
        linear_algebra::sparse::callbacks::dcgn, linear_algebra::sparse::callbacks::dcgs,
        linear_algebra::sparse::callbacks::dgmres, linear_algebra::sparse::callbacks::dir,
        linear_algebra::sparse::callbacks::domn, linear_algebra::sparse::callbacks::sbcg,
        linear_algebra::sparse::callbacks::scg, linear_algebra::sparse::callbacks::scgn,
        linear_algebra::sparse::callbacks::scgs, linear_algebra::sparse::callbacks::sgmres,
        linear_algebra::sparse::callbacks::sir, linear_algebra::sparse::callbacks::somn,
        ode::callbacks::ddeabm, ode::callbacks::dderkf, ode::callbacks::derkf,
        quadrature::callbacks::dqagie, quadrature::callbacks::dqagpe,
        quadrature::callbacks::dqagse, quadrature::callbacks::dqawce,
        quadrature::callbacks::dqawse, quadrature::callbacks::dqc25c,
        quadrature::callbacks::dqc25s, quadrature::callbacks::dqk15, quadrature::callbacks::dqk15i,
        quadrature::callbacks::dqk15w, quadrature::callbacks::dqk21, quadrature::callbacks::dqk31,
        quadrature::callbacks::dqk41, quadrature::callbacks::dqk51, quadrature::callbacks::dqk61,
        quadrature::callbacks::qagie, quadrature::callbacks::qagpe, quadrature::callbacks::qagse,
        quadrature::callbacks::qawce, quadrature::callbacks::qawse, quadrature::callbacks::qc25c,
        quadrature::callbacks::qc25s, quadrature::callbacks::qk15, quadrature::callbacks::qk15i,
        quadrature::callbacks::qk15w, quadrature::callbacks::qk21, quadrature::callbacks::qk31,
        quadrature::callbacks::qk41, quadrature::callbacks::qk51, quadrature::callbacks::qk61,
    };

    #[test]
    fn canonical_paths_compile() {}
}
