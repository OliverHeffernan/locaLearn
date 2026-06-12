use crate::{
    app::{ApplicationContext, StudyCommand},
    generation::{GenerationPipeline, StdoutGenerationLogger},
    providers::ProviderModel,
    study_set::{StudySetCreator, StudySetLayout},
    tui::TerminalApp,
    Result,
};

/// Command that creates a new study set directory.
pub struct CreateStudySetCommand {
    set_name: String,
}

impl CreateStudySetCommand {
    /// Builds a command for the provided study set name.
    pub fn new(set_name: impl Into<String>) -> Self {
        Self {
            set_name: set_name.into(),
        }
    }
}

impl StudyCommand for CreateStudySetCommand {
    fn execute(&self, context: &ApplicationContext) -> Result<()> {
        StudySetCreator::default().create(context.cwd(), &self.set_name)
    }
}

/// Command that generates study artifacts from the current study set.
pub struct GenerateStudySetCommand {
    model: String,
    full_output: bool,
}

impl GenerateStudySetCommand {
    /// Builds a command for the selected provider model.
    pub fn new(model: impl Into<String>, full_output: bool) -> Self {
        Self {
            model: model.into(),
            full_output,
        }
    }
}

impl StudyCommand for GenerateStudySetCommand {
    fn execute(&self, context: &ApplicationContext) -> Result<()> {
        let layout = StudySetLayout::discover(context.cwd())?;
        let model = self.model.parse::<ProviderModel>()?;
        let logger = StdoutGenerationLogger::new(self.full_output);
        GenerationPipeline::new(
            context.provider_registry(),
            context.artifact_registry(),
            &logger,
        )
        .run(&layout, &model)
    }
}

/// Command that lists available provider models.
pub struct ListModelsCommand;

impl StudyCommand for ListModelsCommand {
    fn execute(&self, context: &ApplicationContext) -> Result<()> {
        context
            .provider_registry()
            .model_results()
            .into_iter()
            .filter_map(|result| result.map_err(|error| eprintln!("{error}")).ok())
            .flatten()
            .map(|model| model.selector())
            .for_each(|selector| println!("{selector}"));
        Ok(())
    }
}

/// Command that launches the terminal interface.
pub struct LaunchTuiCommand;

impl StudyCommand for LaunchTuiCommand {
    fn execute(&self, context: &ApplicationContext) -> Result<()> {
        TerminalApp::default().run(context)
    }
}
