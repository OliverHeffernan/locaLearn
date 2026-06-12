use clap::{Parser, Subcommand};

use crate::{app::StudyCommand, cli::commands};

/// Command line arguments for the study application.
#[derive(Debug, Parser)]
#[command(name = "loca")]
#[command(about = "Localearn: create and practice local-first study sets")]
pub struct CliArgs {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Create a new study set directory.
    Create {
        /// Name of the study set directory to create.
        set_name: String,
    },
    /// Generate study materials from resources in the current study set.
    Generate {
        /// Required model selector in `provider/model` format.
        #[arg(short, long)]
        model: String,
        /// Print provider output live while generation runs.
        #[arg(long)]
        full: bool,
    },
    /// List available provider models.
    Models,
    /// Launch the terminal user interface.
    Tui,
}

impl CliArgs {
    /// Parses process arguments into a dynamically dispatched command.
    pub fn command() -> Box<dyn StudyCommand> {
        Self::parse().into_command()
    }

    fn into_command(self) -> Box<dyn StudyCommand> {
        match self.command {
            CliCommand::Create { set_name } => {
                Box::new(commands::CreateStudySetCommand::new(set_name))
            }
            CliCommand::Generate { model, full } => {
                Box::new(commands::GenerateStudySetCommand::new(model, full))
            }
            CliCommand::Models => Box::new(commands::ListModelsCommand),
            CliCommand::Tui => Box::new(commands::LaunchTuiCommand),
        }
    }
}
