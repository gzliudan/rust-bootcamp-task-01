mod jwt;
mod text;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::{jwt::*, text::*};

#[derive(Debug, Parser)]
#[clap(name = "rcli", version , author, about = "The first task for rust bootcamp", long_about = None)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
    #[command(subcommand, about = "jwt sign/verify")]
    Jwt(JwtSubCommand),

    #[command(subcommand, about = "text encrypt/decrypt")]
    Text(TextSubCommand),
}
