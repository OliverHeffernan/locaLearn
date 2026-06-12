use crate::{
    generation::{GenerationLogger, GenerationRequest, StudyContext},
    providers::{DefaultProviderRegistry, ProviderModel},
    study_set::StudySetLayout,
    Result,
};

use super::DefaultArtifactRegistry;

/// Coordinates provider selection and artifact generation.
pub struct GenerationPipeline<'a> {
    provider_registry: &'a DefaultProviderRegistry,
    artifact_registry: &'a DefaultArtifactRegistry,
    logger: &'a dyn GenerationLogger,
}

impl<'a> GenerationPipeline<'a> {
    /// Creates a generation pipeline.
    pub fn new(
        provider_registry: &'a DefaultProviderRegistry,
        artifact_registry: &'a DefaultArtifactRegistry,
        logger: &'a dyn GenerationLogger,
    ) -> Self {
        Self {
            provider_registry,
            artifact_registry,
            logger,
        }
    }

    /// Generates all artifacts configured by the study set metadata.
    pub fn run(&self, layout: &StudySetLayout, model: &ProviderModel) -> Result<()> {
        self.logger.study_set_discovered(layout.root());
        self.logger.model_selected(&model.selector());

        self.logger.metadata_started();
        let metadata = layout.read_metadata()?;
        self.logger.provider_started_for_model(&model.selector());
        let provider = self.provider_registry.create(model)?;
        self.logger.resources_started();
        let context = StudyContext::from_layout(layout);
        self.logger
            .resources_loaded(context.resource_count(), context.loaded_bytes());

        let artifacts = self.artifact_registry.resolve(metadata.artifact_names())?;
        self.logger.artifacts_resolved(artifacts.len());

        artifacts
            .into_iter()
            .try_for_each(|artifact| {
                let name = artifact.name();
                let output_path = artifact.output_path(layout);
                self.logger.artifact_started(name.as_str());
                self.logger.provider_started(name.as_str());
                let request = GenerationRequest::new(artifact.prompt(&context), context.clone());
                provider
                    .generate(request, self.logger.full_output_enabled())
                    .and_then(|response| artifact.write_response(layout, response))?;
                self.logger.artifact_written(name.as_str(), &output_path);
                Ok(())
            })
            .map(|_| self.logger.completed())
    }
}
