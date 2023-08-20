#[derive(thiserror::Error, Debug)]
pub enum DayError {
    #[error("Inquire Error: {0}")]
    InquireError(#[from] inquire::error::InquireError),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

pub trait Configurable {
    fn run_configurator<'a>(&mut self, message: &'a str) -> Result<(), DayError>;
}

pub struct ConfigurableTest {
    
}