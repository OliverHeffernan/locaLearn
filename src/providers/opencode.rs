use std::process::{Command, Stdio};

use crate::{
    generation::{GenerationRequest, GenerationResponse},
    providers::{
        streaming::run_command, GenerationProvider, ProviderFactory, ProviderKind, ProviderModel,
    },
    Result, StudyError,
};

/// Provider implementation that delegates generation to the OpenCode CLI.
pub struct OpenCodeProvider {
    model: String,
    executor: Box<dyn OpenCodeCommandExecutor>,
}

impl Default for OpenCodeProvider {
    fn default() -> Self {
        Self {
            model: String::new(),
            executor: Box::<OpenCodeProcessCommandExecutor>::default(),
        }
    }
}

/// Executes OpenCode commands.
pub trait OpenCodeCommandExecutor {
    /// Runs OpenCode with the selected model and prompt.
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String>;

    /// Lists model identifiers known to OpenCode.
    fn models(&self) -> Result<Vec<String>>;
}

/// Command executor backed by `std::process::Command`.
#[derive(Default)]
pub struct OpenCodeProcessCommandExecutor;

impl OpenCodeCommandExecutor for OpenCodeProcessCommandExecutor {
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String> {
        let mut command = Command::new("opencode");
        command.arg("run").arg("--model").arg(model).arg(prompt);
        run_command(command, full_output)
    }

    fn models(&self) -> Result<Vec<String>> {
        let output = Command::new("opencode")
            .arg("models")
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
            .map(|models| parse_opencode_models(&models))
    }
}

impl ProviderFactory for OpenCodeProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::new("opencode")
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
        Ok(Box::new(OpenCodeProvider {
            model: model.model().to_owned(),
            executor: Box::<OpenCodeProcessCommandExecutor>::default(),
        }))
    }
}

impl GenerationProvider for OpenCodeProvider {
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

fn parse_opencode_models(output: &str) -> Vec<String> {
    output
        .split_whitespace()
        .map(strip_ansi)
        .filter(|token| token.contains('/'))
        .map(|token| token.trim_matches(|character: char| !is_model_character(character)))
        .filter(|token| token.contains('/'))
        .map(str::to_owned)
        .collect()
}

fn strip_ansi(token: &str) -> &str {
    token
        .rsplit('\u{1b}')
        .next()
        .unwrap_or(token)
        .trim_start_matches(|character| character != 'm')
        .trim_start_matches('m')
}

fn is_model_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '/' | '-' | '_' | '.' | ':' | '@')
}
