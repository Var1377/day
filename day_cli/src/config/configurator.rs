use day_core::config::Config;

use super::Configurable;

impl Configurable for Config {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.sleep.run_configurator()?;

        Ok(())
    }

    fn run_optional_configurator(&mut self) -> anyhow::Result<()> {
        if inquire::Confirm::new("Would you like to configure your sleep settings?").with_default(false).prompt()? {
            self.sleep.run_configurator()?;
        }

        Ok(())
    }
}