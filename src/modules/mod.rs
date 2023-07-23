mod commitments;
mod sleep;

trait Module {

}

pub trait Configurable {
    fn run_config(&mut self) -> anyhow::Result<()>;
}