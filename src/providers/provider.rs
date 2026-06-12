use std::str::FromStr;

use crate::{
    generation::{GenerationRequest, GenerationResponse},
    Result, StudyError,
};

/// Stable provider identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderKind(String);

impl ProviderKind {
    /// Creates a provider kind.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the provider kind as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A fully qualified model selector in the form `provider/model`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderModel {
    provider: ProviderKind,
    model: String,
}

impl ProviderModel {
    /// Creates a provider model selector.
    pub fn new(provider: ProviderKind, model: impl Into<String>) -> Self {
        Self {
            provider,
            model: model.into(),
        }
    }

    /// Returns the provider portion of the selector.
    pub fn provider(&self) -> &ProviderKind {
        &self.provider
    }

    /// Returns the provider-specific model identifier.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the selector in `provider/model` format.
    pub fn selector(&self) -> String {
        [self.provider.as_str(), self.model()].join("/")
    }
}

impl FromStr for ProviderModel {
    type Err = StudyError;

    fn from_str(value: &str) -> Result<Self> {
        value
            .split_once('/')
            .filter(|(provider, model)| !provider.is_empty() && !model.is_empty())
            .map(|(provider, model)| Self::new(ProviderKind::new(provider), model))
            .ok_or_else(|| StudyError::InvalidProviderModel(value.to_owned()))
    }
}

/// Produces generated study content from local resources.
pub trait GenerationProvider {
    /// Generates content for the request.
    fn generate(&self, request: GenerationRequest, full_output: bool)
        -> Result<GenerationResponse>;
}

/// Creates providers and describes their model catalog.
pub trait ProviderFactory {
    /// Returns the provider kind created by this factory.
    fn kind(&self) -> ProviderKind;

    /// Lists models available through this provider.
    fn models(&self) -> Result<Vec<ProviderModel>>;

    /// Builds a provider instance for a specific model.
    fn create(&self, model: &ProviderModel) -> Result<Box<dyn GenerationProvider>>;
}

/// Registry of provider factories.
pub struct DefaultProviderRegistry {
    factories: Vec<Box<dyn ProviderFactory>>,
}

impl Default for DefaultProviderRegistry {
    fn default() -> Self {
        Self {
            factories: vec![
                Box::<crate::providers::CopilotProvider>::default(),
                Box::<crate::providers::OpenCodeProvider>::default(),
                Box::<crate::providers::GeminiProvider>::default(),
            ],
        }
    }
}

impl DefaultProviderRegistry {
    /// Lists available models per provider, preserving provider-specific failures.
    pub fn model_results(&self) -> Vec<Result<Vec<ProviderModel>>> {
        self.factories
            .iter()
            .map(Box::as_ref)
            .map(ProviderFactory::models)
            .collect()
    }

    /// Lists all available models in `provider/model` format.
    pub fn models(&self) -> Result<Vec<ProviderModel>> {
        self.model_results()
            .into_iter()
            .collect::<Result<Vec<_>>>()
            .map(|models| models.into_iter().flatten().collect())
    }

    /// Creates the provider selected by the model selector.
    pub fn create(&self, model: &ProviderModel) -> Result<Box<dyn GenerationProvider>> {
        self.factories
            .iter()
            .map(Box::as_ref)
            .find(|factory| factory.kind() == *model.provider())
            .ok_or_else(|| StudyError::ProviderNotFound(model.provider().as_str().to_owned()))
            .and_then(|factory| factory.create(model))
    }
}
