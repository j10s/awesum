mod blake2b;
mod blake2s;
mod crc32;

use super::Algorithm;
use anyhow::Error;
use blake2b::Blake2b;
use blake2s::Blake2s;
use crc32::Crc32;
use digest::{Digest, DynDigest};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::io::Read;

pub fn hash<R: Read>(reader: &mut R, algorithm: Algorithm) -> Result<String, Error> {
    let mut buffer = [0u8; 8192];
    let mut hasher = hasher(algorithm);

    loop {
        let length = reader.read(&mut buffer[..])?;
        if length == 0 {
            break;
        }

        hasher.update(&buffer[..length]);
    }

    let result = hasher.finalize();
    let hash = format(&*result, algorithm);
    Ok(hash)
}

fn hasher(algorithm: Algorithm) -> Box<dyn DynDigest> {
    match algorithm {
        Algorithm::CRC32 => Box::new(Crc32::new()),
        Algorithm::MD5 => Box::new(Md5::new()),
        Algorithm::SHA1 => Box::new(Sha1::new()),
        Algorithm::SHA2224 => Box::new(Sha224::new()),
        Algorithm::SHA2256 => Box::new(Sha256::new()),
        Algorithm::SHA2384 => Box::new(Sha384::new()),
        Algorithm::SHA2512 => Box::new(Sha512::new()),
        Algorithm::SHA3224 => Box::new(Sha3_224::new()),
        Algorithm::SHA3256 => Box::new(Sha3_256::new()),
        Algorithm::SHA3384 => Box::new(Sha3_384::new()),
        Algorithm::SHA3512 => Box::new(Sha3_512::new()),
        Algorithm::BLAKE2B => Box::new(Blake2b::new()),
        Algorithm::BLAKE2S => Box::new(Blake2s::new()),
        Algorithm::BLAKE3 => Box::new(blake3::Hasher::new()),
    }
}

fn format(hash: &[u8], algorithm: Algorithm) -> String {
    if matches!(algorithm, Algorithm::CRC32) {
        return hex::encode_upper(hash);
    }

    return hex::encode(hash);
}
