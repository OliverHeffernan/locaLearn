use std::process::Command;

use crate::{
    generation::{GenerationRequest, GenerationResponse},
    providers::{
        streaming::run_command, GenerationProvider, ProviderFactory, ProviderKind, ProviderModel,
    },
    Result,
};

/// Provider implementation that delegates generation to the Gemini CLI.
pub struct GeminiProvider {
    model: String,
    executor: Box<dyn GeminiCommandExecutor>,
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self {
            model: "gemini-2.5-pro".to_owned(),
            executor: Box::<GeminiProcessCommandExecutor>::default(),
        }
    }
}

/// Executes Gemini commands.
pub trait GeminiCommandExecutor {
    /// Runs Gemini with the selected model and prompt.
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String>;

    /// Lists model identifiers available through the Gemini adapter.
    fn models(&self) -> Result<Vec<String>>;
}

/// Command executor backed by `std::process::Command`.
#[derive(Default)]
pub struct GeminiProcessCommandExecutor;

impl GeminiCommandExecutor for GeminiProcessCommandExecutor {
    fn execute(&self, model: &str, prompt: &str, full_output: bool) -> Result<String> {
        let mut command = Command::new("gemini");
        command
            .arg("--skip-trust")
            .arg("--model")
            .arg(model)
            .arg("--output-format")
            .arg("text")
            .arg(prompt);
        run_command(command, full_output)
    }

    fn models(&self) -> Result<Vec<String>> {
        Ok(GEMINI_MODELS.iter().map(ToString::to_string).collect())
    }
}

impl ProviderFactory for GeminiProvider {
    fn kind(&self) -> ProviderKind {
        ProviderKind::new("gemini")
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
        Ok(Box::new(GeminiProvider {
            model: model.model().to_owned(),
            executor: Box::<GeminiProcessCommandExecutor>::default(),
        }))
    }
}

impl GenerationProvider for GeminiProvider {
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

const GEMINI_MODELS: &[&str] = &[
    "gemini-2.5-pro",
    "gemini-2.5-flash",
    "gemini-2.5-flash-lite",
    "gemini-2.0-flash",
];
