use day_core::modules::sleep::Biphasic;
use crate::config::Configurable;

impl Configurable for Biphasic {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}