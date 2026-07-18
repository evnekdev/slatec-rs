//! Narrow link probe for the reviewed real FFTPACK `RFFTI/RFFTF/RFFTB` closure.

use slatec::fftpack::RealFftPlan;

fn main() {
    let mut values = [1.0_f32, 0.0, -1.0, 0.0];
    let mut plan = RealFftPlan::new(values.len()).expect("valid real FFT plan");
    plan.forward(&mut values).expect("RFFTF");
    plan.backward(&mut values).expect("RFFTB");
}
