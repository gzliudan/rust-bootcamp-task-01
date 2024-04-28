use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

pub fn encrypt_text(key: &str, msg: &str) -> Result<String> {
    let bin_key = make_binary_key(key)?;
    let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(&bin_key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(&nonce, msg.as_bytes())
        .map_err(|e| anyhow::anyhow!(e))?;
    Ok(STANDARD.encode(ciphertext))
}

pub fn decrypt_text(key: &str, msg: &str) {
    println!("decrypt text: key={}, msg={}", key, msg);
}

pub fn generate_text_key() -> String {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    "0x".to_string() + &hex::encode_upper(key)
}

pub fn make_binary_key(key: &str) -> Result<[u8; 32]> {
    let key = key.trim_start_matches("0x").trim_start_matches("0X");
    let key_len = key.len();
    match key_len {
        64 => {
            let decoded = hex::decode(key)?;
            let ret: [u8; 32] = decoded.as_slice().try_into()?;
            Ok(ret)
        }
        _ => Err(anyhow::anyhow!("key length must be 64, but got {key_len}")),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn text_generate_text_key() -> Result<()> {
        let txt_key = generate_text_key();
        let bin_key = make_binary_key(&txt_key)?;
        let encoded = hex::encode_upper(bin_key);
        let decoded = hex::decode(&encoded)?;
        assert_eq!(txt_key, "0x".to_string() + &encoded);
        assert_eq!(bin_key.as_slice(), decoded);
        Ok(())
    }
}
