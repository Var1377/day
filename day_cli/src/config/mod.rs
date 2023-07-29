mod cli;
mod configurator;
pub use cli::ConfigCli;

pub use cli::ConfigLoad;

pub trait Configurable {
    fn run_configurator(&mut self) -> anyhow::Result<()>;

    fn run_optional_configurator(&mut self) -> anyhow::Result<()> {
        self.run_configurator()
    }
}
