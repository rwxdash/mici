use crate::{cli::schemas::v1::CommandSchema, runner::context::ExecutionContext};

pub struct Coordinator<'a> {
    context: ExecutionContext<'a>,
    schema: &'a CommandSchema,
}

impl<'a> Coordinator<'a> {
    pub fn new(context: ExecutionContext<'a>, schema: &'a CommandSchema) -> Self {
        Self { context, schema }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // println!("{:?}", self.context.inputs);
        // println!("{:?}", self.schema);

        println!("> Starting execution of: {}", self.schema.name);

        if let Some(description) = &self.schema.description {
            println!("  {}", description);
        }

        println!("> Executing {} steps", self.schema.steps.len());

        for (index, step) in self.schema.steps.iter().enumerate() {
            println!("> {}/{}: {}", index + 1, self.schema.steps.len(), step.name);

            // TODO: check when/always condition
            // Exec

            println!("  Step completed: {}", step.name);
            println!()
        }

        println!("Done!");
        Ok(())
    }
}
