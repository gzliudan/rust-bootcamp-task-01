use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{decrypt_text, encrypt_text, generate_text_key, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(name = "encrypt", about = "Encrypt a plain message with key")]
    Encrypt(TextEncryptArgs),

    #[command(name = "decrypt", about = "Decrypt a encrypted message with key")]
    Decrypt(TextDecryptArgs),

    #[command(name = "gen-key", about = "Generate a key")]
    Generate(TextGenKeyArgs),
}

#[derive(Debug, Parser)]
pub struct TextEncryptArgs {
    /// The message to sign
    #[arg(long, help = "The key for encrypt/decrypt")]
    pub key: String,

    #[arg()]
    pub message: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptArgs {
    /// The message to sign
    #[arg(long, help = "The key for encrypt/decrypt")]
    pub key: String,

    #[arg()]
    pub message: String,
}

#[derive(Debug, Parser)]
pub struct TextGenKeyArgs {}

impl CmdExector for TextEncryptArgs {
    fn execute(self) -> anyhow::Result<()> {
        let result = encrypt_text(&self.key, &self.message)?;
        println!("encrypt result: {result}");
        Ok(())
    }
}

impl CmdExector for TextDecryptArgs {
    fn execute(self) -> anyhow::Result<()> {
        let result = decrypt_text(&self.key, &self.message)?;
        println!("decrypt result: {result}");
        Ok(())
    }
}

impl CmdExector for TextGenKeyArgs {
    fn execute(self) -> anyhow::Result<()> {
        let key = generate_text_key();
        println!("text key: {}", key);
        Ok(())
    }
}
