use std::path::Path;

/// Observes generation progress without coupling the pipeline to a UI.
pub trait GenerationLogger {
    /// Logs a human-readable progress message.
    fn log(&self, message: &str);

    /// Returns whether raw provider output should be streamed live.
    fn full_output_enabled(&self) -> bool {
        false
    }

    /// Logs that a study set has been discovered.
    fn study_set_discovered(&self, root: &Path) {
        self.log(&format!("Found study set at {}", root.display()));
    }

    /// Logs the selected model.
    fn model_selected(&self, selector: &str) {
        self.log(&format!("Using model {selector}"));
    }

    /// Logs that study metadata is being loaded.
    fn metadata_started(&self) {
        self.log("Loading study metadata");
    }

    /// Logs that the provider is being prepared.
    fn provider_started_for_model(&self, selector: &str) {
        self.log(&format!("Preparing provider for {selector}"));
    }

    /// Logs that local resources are being scanned.
    fn resources_started(&self) {
        self.log("Scanning local resources");
    }

    /// Logs loaded resource statistics.
    fn resources_loaded(&self, count: usize, bytes: usize) {
        self.log(&format!("Loaded {count} resource file(s), {bytes} byte(s)"));
    }

    /// Logs how many artifacts will be generated.
    fn artifacts_resolved(&self, count: usize) {
        self.log(&format!("Preparing to generate {count} artifact(s)"));
    }

    /// Logs the start of one artifact.
    fn artifact_started(&self, name: &str) {
        self.log(&format!("Generating {name}"));
    }

    /// Logs that a provider request is in progress.
    fn provider_started(&self, name: &str) {
        self.log(&format!("Calling provider for {name}"));
    }

    /// Logs where an artifact was written.
    fn artifact_written(&self, name: &str, path: &Path) {
        self.log(&format!("Wrote {name} to {}", path.display()));
    }

    /// Logs that generation has completed.
    fn completed(&self) {
        self.log("Generation complete");
    }
}

/// Logger that writes progress to standard error.
#[derive(Default)]
pub struct StdoutGenerationLogger {
    full_output: bool,
}

impl StdoutGenerationLogger {
    /// Creates a logger with optional provider output streaming enabled.
    pub fn new(full_output: bool) -> Self {
        Self { full_output }
    }
}

impl GenerationLogger for StdoutGenerationLogger {
    fn log(&self, message: &str) {
        eprintln!("[loca generate] {message}");
    }

    fn full_output_enabled(&self) -> bool {
        self.full_output
    }
}
