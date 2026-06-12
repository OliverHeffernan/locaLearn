use std::process::{Command, Stdio};

use crate::{
    generation::{GenerationRequest, GenerationResponse},
    providers::{
        streaming::run_command, GenerationProvider, ProviderFactory, ProviderKind, ProviderModel,
    },
    Result, StudyError,
};

/// Executes provider commands.
pub trait CommandExecutor {
    /// Runs a command with the provided prompt.
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String>;

    /// Lists models exposed by this command.
    fn models(&self) -> Result<Vec<String>>;
}

/// Command executor backed by `std::process::Command`.
#[derive(Default)]
pub struct ProcessCommandExecutor;

impl CommandExecutor for ProcessCommandExecutor {
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String> {
        let mut command = Command::new("copilot");
        command
            .arg("--no-color")
            .arg("--model")
            .arg(model)
            .arg("-p")
            .arg(prompt);
        run_command(command, full_output)
    }

    fn models(&self) -> Result<Vec<String>> {
        let output = Command::new("copilot")
            .arg("--help")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).to_string())
            .ok_or_else(|| {
                StudyError::ProviderFailed(String::from_utf8_lossy(&output.stderr).to_string())
            })
            .map(|help| parse_copilot_models(&help))
    }
}

/// Provider implementation that delegates generation to a Copilot-compatible CLI.
pub struct CopilotProvider {
    model: String,
    executor: Box<dyn CommandExecutor>,
}

impl Default for CopilotProvider {
    fn default() -> Self {
        Self {
            model: "gpt-5".to_owned(),
            executor: Box::<ProcessCommandExecutor>::default(),
        }
    }
}

impl ProviderFactory for CopilotProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::new("copilot")
    }

    fn models(&self) -> Result<Vec<ProviderModel>> {
        self.executor.models().map(|models| {
            models
                .into_iter()
                .map(|model| ProviderModel::new(self.kind(), model))
                .collect()
        })
    }

    fn create(&self, model: &ProviderModel) -> Result<Box<dyn GenerationProvider>> {
        Ok(Box::new(CopilotProvider {
            model: model.model().to_owned(),
            executor: Box::<ProcessCommandExecutor>::default(),
        }))
    }
}

impl GenerationProvider for CopilotProvider {
    fn generate(
        &self,
        request: GenerationRequest,
        full_output: bool,
    ) -> Result<GenerationResponse> {
        self.executor
            .execute(&self.model, &request.prompt(), full_output)
            .map(GenerationResponse::new)
    }
}

fn parse_copilot_models(help: &str) -> Vec<String> {
    help.split("(choices:")
        .nth(1)
        .and_then(|choices| choices.split(')').next())
        .into_iter()
        .flat_map(|choices| choices.split('"').skip(1).step_by(2))
        .map(str::to_owned)
        .collect()
}
