pub struct HelpCommand {
    pub command_name: String,
}

const POSSIBLE_COMMAND_NAMES: [&str; 2] = ["init", "config"];

impl HelpCommand {
    pub fn run(&self) {
        if !POSSIBLE_COMMAND_NAMES
            .iter()
            .any(|&v| v == self.command_name)
        {
            println!("This is not a mapped command!");
            std::process::exit(1);
        }

        match &self.command_name[..] {
            "init" => Self::help_init(),
            "config" => Self::help_config(),
            _ => (),
        }
    }

    fn help_init() {
        println!("init ...");
        // TODO: create the init help message
    }

    fn help_config() {
        println!("config ...");
        // TODO: create the config help message
    }
}
