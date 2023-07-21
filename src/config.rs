use clap::Args;

#[derive(Args, Debug)]
pub struct ConfigArgs {}

#[derive(Debug)]
pub struct Config {
    /// time breaking / time working
    pub break_ratio: f64
}

impl Config {
    pub fn new() -> Self {
        Config {
            break_ratio: 0.15
        }
    }
}