use crate::lib::maintenance::base_command::BaseCommand;

pub struct InitCommand {
    base: BaseCommand,
}

impl InitCommand {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        usage: &'static str,
        short_usage: &'static str,
    ) -> Self {
        Self {
            base: BaseCommand {
                name,
                description,
                usage,
                short_usage,
            },
        }
    }

    pub fn run(&self) {
        println!("inside init cmd");
    }
}

pub const INIT_COMMAND: InitCommand = InitCommand::new("hellow", "hellow", "hellow", "hellow");
