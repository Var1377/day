use ptree::{TreeBuilder, print_tree};

use crate::Runnable;

#[derive(Debug, clap::Args)]
pub struct Args {}

impl Runnable for Args {
    type Args = ();

    fn run(&self, _args: &Self::Args, state: &mut day_core::state::State) -> anyhow::Result<()> {
        let mut builder = TreeBuilder::new("Slices".to_string());

        for slice in state.slices.iter() {
            build_tree(&slice.inner, &mut builder);
        }

        let tree = builder.build();

        print_tree(&tree)?;

        Ok(())
    }
}

fn build_tree(slice: &day_core::state::SliceInner, builder: &mut TreeBuilder) {
    builder.begin_child(slice.id.clone());

    for child in slice.children.iter() {
        build_tree(&child.inner, builder);
    }

    builder.end_child();
}