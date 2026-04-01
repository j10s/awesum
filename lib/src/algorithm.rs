use strum::{EnumString, VariantNames};

#[derive(Clone, Copy, Debug, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Algorithm {
    CRC32,
    MD5,
    SHA1,
    SHA2224,
    SHA2256,
    SHA2384,
    SHA2512,
    SHA3224,
    SHA3256,
    SHA3384,
    SHA3512,
    BLAKE2B,
    BLAKE2S,
    BLAKE3,
}
