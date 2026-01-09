use std::process::ExitCode;

macro_rules! define_commands {
    ( $( $variant:ident => $module:ident ),+ $(,)? ) => {
        $( pub mod $module; )+

        #[derive(clap::Subcommand, Debug)]
        pub enum Command {
            $( $variant($module::Args), )+
        }

        impl Command {
            pub fn run(self) -> ExitCode {
                match self {
                    $( Command::$variant(args) => $module::run(args), )+
                }
            }
        }
    };
}

define_commands! {
    EmptyCols => empty_cols,
}
