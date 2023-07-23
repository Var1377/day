mod cli;

pub use cli::ConfigCli;

#[derive(Debug)]
pub struct Config {
    /// time breaking / time working
    pub break_ratio: f64
}

impl Config {

}

impl Default for Config {
    fn default() -> Self {
        Self {
            break_ratio: 0.15,
        }
    }
}