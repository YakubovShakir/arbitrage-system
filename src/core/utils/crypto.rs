use crate::core::types::{HmacSha256, HmacSha512};
use base64::{Engine, engine::general_purpose};
use data_encoding::BASE64;
use hmac::Mac;
use ring::{
    digest,
    rand::SystemRandom,
    signature::{RSA_PKCS1_SHA256, RsaKeyPair},
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub fn sha512_with_ring_return_hex(input: &str) -> String {
    let digest = digest::digest(&digest::SHA512, input.as_bytes());
    hex::encode(digest.as_ref())
}
pub fn encrypt_hmac_sha256(
    secret_key: &str,
    signature: &str,
) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    Ok(HmacSha256::new_from_slice(secret_key.as_bytes())?
        .chain_update(signature.as_bytes())
        .finalize()
        .into_bytes()
        .into())
}

pub fn encrypt_hmac_sha512(
    secret_key: &str,
    signature: &str,
) -> Result<[u8; 64], Box<dyn std::error::Error>> {
    Ok(HmacSha512::new_from_slice(secret_key.as_bytes())?
        .chain_update(signature.as_bytes())
        .finalize()
        .into_bytes()
        .into())
}

pub fn hex_encode<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
}

pub fn sign_rsa_sha256(
    private_key_str: &str,
    message: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Декодируем base64 приватного ключа
    let der_bytes = general_purpose::STANDARD.decode(private_key_str)?;

    // Создаем RSA ключевую пару с явным преобразованием ошибки
    let key_pair = Arc::new(
        RsaKeyPair::from_pkcs8(&der_bytes).map_err(|e| format!("Invalid private key: {:?}", e))?,
    );

    let rng = SystemRandom::new();

    // Вычисляем SHA256 хеш сообщения
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let digest = hasher.finalize();

    // Получаем длину модуля
    let modulus_len = key_pair.public().modulus_len();

    // Подписываем с явным преобразованием ошибки
    let mut signature = vec![0; modulus_len];
    key_pair
        .sign(&RSA_PKCS1_SHA256, &rng, &digest, &mut signature)
        .map_err(|e| format!("Signing failed: {:?}", e))?;

    Ok(signature)
}
pub fn base64_encode(data: &[u8]) -> String {
    BASE64.encode(data)
}
