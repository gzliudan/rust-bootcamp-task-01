use clap::Parser;
use rcli::{CmdArgs, CmdExector};

fn main() -> anyhow::Result<()> {
    let opts = CmdArgs::parse();
    opts.cmd.execute()
}
