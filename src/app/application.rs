use std::{env, path::PathBuf};

use crate::{
    cli::CliArgs, generation::DefaultArtifactRegistry, providers::DefaultProviderRegistry, Result,
};

/// Runtime dependencies shared by commands and UI flows.
pub struct ApplicationContext {
    cwd: PathBuf,
    provider_registry: DefaultProviderRegistry,
    artifact_registry: DefaultArtifactRegistry,
}

impl ApplicationContext {
    /// Builds application context rooted at the process working directory.
    pub fn from_current_dir() -> Result<Self> {
        Ok(Self {
            cwd: env::current_dir()?,
            provider_registry: DefaultProviderRegistry::default(),
            artifact_registry: DefaultArtifactRegistry::default(),
        })
    }

    /// Returns the directory from which the application was started.
    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    /// Returns the configured provider registry.
    pub fn provider_registry(&self) -> &DefaultProviderRegistry {
        &self.provider_registry
    }

    /// Returns the configured artifact registry.
    pub fn artifact_registry(&self) -> &DefaultArtifactRegistry {
        &self.artifact_registry
    }
}

/// Entry point for the study application.
#[derive(Default)]
pub struct Application;

impl Application {
    /// Parses user intent and delegates to the command implementation.
    pub fn run(&self) -> Result<()> {
        let context = ApplicationContext::from_current_dir()?;
        CliArgs::command().execute(&context)
    }
}
