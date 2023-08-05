pub mod commitments;
pub mod custom;
pub mod sleep;
pub mod task;
pub mod todos;

pub enum Module {
    Parent { children: Vec<Module> },
    TaskList {},
    Flexible {},
}