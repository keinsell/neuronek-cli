use async_std::task;
use miette::set_panic_hook;

mod cli {
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;
    use substance::SubstanceCommands;

    pub(super) mod substance {
        use clap::{Parser, Subcommand};

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct CreateSubstance {
            #[arg(short, long)]
            name: String,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct UpdateSubstance {
            #[arg(short, long)]
            id: String,
            #[arg(short, long)]
            pub name: Option<String>,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct DeleteSubstance {
            #[arg(short, long)]
            pub id: String,
        }

        #[derive(Subcommand)]
        pub enum SubstanceCommands {
            Create(CreateSubstance),
            Update(UpdateSubstance),
            Delete(DeleteSubstance),
        }

        #[derive(Parser)]
        #[command(args_conflicts_with_subcommands = true)]
        pub(crate) struct SubstanceCommand {
            #[command(subcommand)]
            pub command: SubstanceCommands,
        }
    }

    #[derive(Subcommand)]
    pub(super) enum ProgramCommand {
        Substance(substance::SubstanceCommand),
    }

    #[derive(Parser)]
    #[command(
        version = "0.0.1-dev",
        about = "Dosage journal that knows!",
        long_about = "🧬 Intelligent dosage tracker application with purpose to monitor supplements, nootropics and psychoactive substances along with their long-term influence on one's mind and body."
    )]
    pub(super) struct Program {
        /// Optional name to operate on
        pub name: Option<String>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        pub config: Option<PathBuf>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub debug: u8,

        #[command(subcommand)]
        pub command: ProgramCommand,
    }

    pub(super) async fn run_program() {
        let cli = Program::parse();

        match cli.command {
            ProgramCommand::Substance(substance_command) => match substance_command.command {
                SubstanceCommands::Create(_create_sibstance) => {
                    todo!()
                }
                SubstanceCommands::Delete(_delete_substance) => {
                    todo!()
                }
                SubstanceCommands::Update(_update_substance) => {
                    todo!()
                }
            },
        }
    }
}

fn main() {
    // set_hook();
    set_panic_hook();

    task::block_on(async {
        cli::run_program().await;
    });
}
