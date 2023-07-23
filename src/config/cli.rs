use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ConfigCli {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

impl ConfigCli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.subcmd {
            Some(subcmd) => match subcmd {
                _ => todo!()
            },
            None => {
                todo!()
            }
        };
    }
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    // Sleep(SleepArgs),
}
