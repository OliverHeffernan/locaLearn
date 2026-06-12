use serde::{Deserialize, Serialize};

/// Persistent study set metadata stored in `study.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySetMetadata {
    name: String,
    version: u32,
    provider: ProviderConfiguration,
    generation: GenerationConfiguration,
}

impl StudySetMetadata {
    /// Creates default metadata for a new study set.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: 1,
            provider: ProviderConfiguration::default(),
            generation: GenerationConfiguration::default(),
        }
    }

    /// Returns the study set name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns configured provider information.
    pub fn provider(&self) -> &ProviderConfiguration {
        &self.provider
    }

    /// Returns the artifact names selected for generation.
    pub fn artifact_names(&self) -> impl Iterator<Item = &str> {
        self.generation.artifacts.iter().map(String::as_str)
    }
}

/// Provider configuration for a study set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfiguration {
    kind: String,
}

impl ProviderConfiguration {
    /// Returns the provider kind.
    pub fn kind(&self) -> &str {
        &self.kind
    }
}

impl Default for ProviderConfiguration {
    fn default() -> Self {
        Self {
            kind: "copilot".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenerationConfiguration {
    artifacts: Vec<String>,
}

impl Default for GenerationConfiguration {
    fn default() -> Self {
        Self {
            artifacts: [
                "flashcards",
                "multiple_choice",
                "fill_blanks",
                "practice_test",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
}
