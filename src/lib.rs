mod cli;
mod process;

use enum_dispatch::enum_dispatch;

pub use self::{cli::*, process::*};

#[enum_dispatch]
pub trait CmdExector {
    fn execute(self) -> anyhow::Result<()>;
}
