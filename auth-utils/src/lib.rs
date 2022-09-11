use digest::{
    block_buffer::Eager,
    core_api::{BufferKindUser, CoreProxy, FixedOutputCore, UpdateCore},
    crypto_common::BlockSizeUser,
    typenum::{IsLess, Le, NonZero, U256},
    HashMarker,
};
use hmac::{Hmac, Mac};
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

pub enum HashAlgorithm {
    Sha2_224,
    Sha2_256,
    Sha2_512_224,
    Sha2_512_256,
    Sha2_384,
    Sha2_512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

pub fn sign(
    data: String,
    secret: String,
    hash_algorithm: HashAlgorithm,
) -> Result<String, anyhow::Error> {
    match hash_algorithm {
        HashAlgorithm::Sha2_224 => signature::<Sha224>(data, secret),
        HashAlgorithm::Sha2_256 => signature::<Sha256>(data, secret),
        HashAlgorithm::Sha2_512_224 => signature::<Sha512_224>(data, secret),
        HashAlgorithm::Sha2_512_256 => signature::<Sha512_256>(data, secret),
        HashAlgorithm::Sha2_384 => signature::<Sha384>(data, secret),
        HashAlgorithm::Sha2_512 => signature::<Sha512>(data, secret),
        HashAlgorithm::Sha3_224 => signature::<Sha3_224>(data, secret),
        HashAlgorithm::Sha3_256 => signature::<Sha3_256>(data, secret),
        HashAlgorithm::Sha3_384 => signature::<Sha3_384>(data, secret),
        HashAlgorithm::Sha3_512 => signature::<Sha3_512>(data, secret),
    }
}

fn signature<T>(data: String, secret: String) -> Result<String, anyhow::Error>
where
    T: CoreProxy,
    T::Core: HashMarker
        + UpdateCore
        + FixedOutputCore
        + BufferKindUser<BufferKind = Eager>
        + Default
        + Clone,
    <T::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<T::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    let mut hmac = Hmac::<T>::new_from_slice(secret.as_bytes())?;
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    let hex = hex::encode(signature);
    Ok(hex)
}
