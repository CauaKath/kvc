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
            let non_mapped_command_msg = "This is not a mapped command!".to_owned()
                + " Try one of the following:"
                + "\n\n"
                + "- kvc help init"
                + "\n"
                + "- kvc help config";

            println!("{}", non_mapped_command_msg);
            std::process::exit(1);
        }

        match &self.command_name[..] {
            "init" => Self::help_init(),
            "config" => Self::help_config(),
            _ => (),
        }
    }

    fn help_init() {
        let help_init_msg =
            "The init command will create the base structure of the kvc repository in the root of the folder you’re currently in.".to_owned() +
            " It can be used like this:" +
            "\n\n" + 
            "kvc init" +
            "\n\n" +
            "If the command is used in a folder that is a kvc repository already," +
            " it’ll display a message and simply do nothing.";

        print!("{}", help_init_msg);
    }

    fn help_config() {
        let help_config_msg =
            "The config command is used to access or change a configuration from your repository."
                .to_owned()
                + " The available configurations at the moment are:"
                + "\n\n"
                + "- base_branch"
                + "\n"
                + "- user.name"
                + "\n"
                + "- user.email"
                + "\n"
                + "- list"
                + "\n\n"
                + "The command can be used in the following ways:"
                + "\n\n"
                + "kvc config {available_config} -> show the requested config value"
                + "\n"
                + "kvc config {available_config} {value} -> change the value of the passed config"
                + "\n"
                + "kvc config list -> print all the configuration file";

        println!("{}", help_config_msg);
    }
}
