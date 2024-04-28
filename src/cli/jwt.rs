use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{sign_jwt, verify_jwt, CmdExector};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign a JWT")]
    Sign(JwtSignArgs),

    #[command(name = "verify", about = "Verify a signed JWT")]
    Verify(JwtVerifyArgs),
}

#[derive(Debug, Parser)]
pub struct JwtSignArgs {
    #[arg(long, help = "subject")]
    pub sub: String,

    #[arg(long, help = "audience")]
    pub aud: String,

    #[arg(long, help = "expiration time")]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyArgs {
    #[arg(help = "token")]
    pub token: String,
}

impl CmdExector for JwtSignArgs {
    fn execute(self) -> anyhow::Result<()> {
        let result = sign_jwt(self.aud, self.sub, self.exp)?;
        println!("sign result: {result}");
        Ok(())
    }
}

impl CmdExector for JwtVerifyArgs {
    fn execute(self) -> anyhow::Result<()> {
        let result = verify_jwt(self.token)?;
        println!("verify result: {result}");
        Ok(())
    }
}
