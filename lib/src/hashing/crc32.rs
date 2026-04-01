// Credit to https://github.com/ajungren/crc32_digest/pull/2 with minor modifications

use crc32fast::Hasher;
use digest::consts::U4;
use digest::{FixedOutput, FixedOutputReset, HashMarker, Output, OutputSizeUser, Reset, Update};

pub use digest::Digest;

#[derive(Clone, Default)]
/// Wraps a [`Hasher`] and provides it with [`Digest`] and [`DynDigest`] implementations.
///
/// [`Digest`]: ../digest/trait.Digest.html
/// [`DynDigest`]: ../digest/trait.DynDigest.html
/// [`Hasher`]: ../crc32fast/struct.Hasher.html
pub struct Crc32(Hasher);

impl Crc32 {
    /// Creates a new `Crc32`.
    #[inline]
    pub fn new() -> Self {
        Self(Hasher::new())
    }
}

impl HashMarker for Crc32 {}

impl Update for Crc32 {
    #[inline]
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl OutputSizeUser for Crc32 {
    type OutputSize = U4;
}

impl FixedOutput for Crc32 {
    #[inline]
    fn finalize_into(self, out: &mut Output<Self>) {
        // FixedOutput trait requires that the output is written into the given buffer of bytes
        // but crc32fast::Hasher::finalize() returns a u32, so we have to convert it
        let result = self.0.finalize();
        let bytes = result.to_be_bytes();
        out.copy_from_slice(&bytes);
    }
}

impl Reset for Crc32 {
    #[inline]
    fn reset(&mut self) {
        self.0.reset();
    }
}

impl FixedOutputReset for Crc32 {
    #[inline]
    fn finalize_into_reset(&mut self, _out: &mut Output<Self>) {
        unimplemented!();
    }
}
