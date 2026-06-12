use crate::{app::ApplicationContext, Result};

/// Executable unit created from CLI input or future UI actions.
pub trait StudyCommand {
    /// Runs the command with access to application services.
    fn execute(&self, context: &ApplicationContext) -> Result<()>;
}

/// Thin wrapper that executes boxed commands.
#[derive(Default)]
pub struct CommandRouter;

impl CommandRouter {
    /// Executes a dynamically dispatched command.
    pub fn route(
        &self,
        command: Box<dyn StudyCommand>,
        context: &ApplicationContext,
    ) -> Result<()> {
        command.execute(context)
    }
}
