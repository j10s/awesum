use blake2s_simd::{Params, State};
use digest::consts::U32;
use digest::{FixedOutput, FixedOutputReset, HashMarker, Output, OutputSizeUser, Reset, Update};

#[derive(Clone, Default)]
pub struct Blake2s(State);

impl Blake2s {
    /// Creates a new `Blake2s`.
    #[inline]
    pub fn new() -> Self {
        Self(Params::default().to_state())
    }
}

impl HashMarker for Blake2s {}

impl Update for Blake2s {
    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl OutputSizeUser for Blake2s {
    type OutputSize = U32;
}

impl FixedOutput for Blake2s {
    #[inline]
    fn finalize_into(self, out: &mut Output<Self>) {
        let result = self.0.finalize();
        out.copy_from_slice(&result.as_bytes());
    }
}

impl Reset for Blake2s {
    #[inline]
    fn reset(&mut self) {
        self.0 = Params::default().to_state();
    }
}

impl FixedOutputReset for Blake2s {
    #[inline]
    fn finalize_into_reset(&mut self, _out: &mut Output<Self>) {
        unimplemented!();
    }
}
