use crate::{
    cli::schemas::{self, v1::CommandSchema},
    runner::context::ExecutionContext,
};

pub struct Runner<'a> {
    context: ExecutionContext<'a>,
    schema: &'a CommandSchema,
}

impl<'a> Runner<'a> {
    pub fn new(context: ExecutionContext<'a>, schema: &'a CommandSchema) -> Self {
        Self { context, schema }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
