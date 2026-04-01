use blake2b_simd::{Params, State};
use digest::consts::U64;
use digest::{FixedOutput, FixedOutputReset, HashMarker, Output, OutputSizeUser, Reset, Update};

#[derive(Clone, Default)]
pub struct Blake2b(State);

impl Blake2b {
    /// Creates a new `Blake2b`.
    #[inline]
    pub fn new() -> Self {
        Self(Params::default().to_state())
    }
}

impl HashMarker for Blake2b {}

impl Update for Blake2b {
    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl OutputSizeUser for Blake2b {
    type OutputSize = U64;
}

impl FixedOutput for Blake2b {
    #[inline]
    fn finalize_into(self, out: &mut Output<Self>) {
        let result = self.0.finalize();
        out.copy_from_slice(&result.as_bytes());
    }
}

impl Reset for Blake2b {
    #[inline]
    fn reset(&mut self) {
        self.0 = Params::default().to_state();
    }
}

impl FixedOutputReset for Blake2b {
    #[inline]
    fn finalize_into_reset(&mut self, _out: &mut Output<Self>) {
        unimplemented!();
    }
}
