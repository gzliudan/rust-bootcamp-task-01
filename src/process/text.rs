use anyhow::Result;

use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305,
};

// 文本加密解密
pub fn encrypt_text(key: &str, msg: &str) {
    println!("encrypt text: key={}, msg={}", key, msg);
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
