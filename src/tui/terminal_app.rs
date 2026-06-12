use std::{io, time::Duration};

use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{
    app::ApplicationContext,
    study_set::StudySetLayout,
    tui::{
        FlashcardDeck, FlashcardStudyScreen, ModeEntry, MultipleChoiceDeck, MultipleChoiceScreen,
        Screen, ScreenTransition, StudyMode, StudyWorkspaceScreen,
    },
    Result,
};

/// Terminal application runner.
#[derive(Default)]
pub struct TerminalApp;

impl TerminalApp {
    /// Starts the TUI event loop.
    pub fn run(&self, context: &ApplicationContext) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let mut screen = self.initial_screen(context);

        let result = self.run_loop(&mut terminal, screen.as_mut());

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        result
    }

    fn initial_screen(&self, context: &ApplicationContext) -> Box<dyn Screen> {
        StudySetLayout::discover(context.cwd())
            .map(|layout| Box::new(self.workspace_screen(&layout)) as Box<dyn Screen>)
            .unwrap_or_else(|error| Box::new(StudyWorkspaceScreen::dashboard(error.to_string())))
    }

    fn workspace_screen(&self, layout: &StudySetLayout) -> StudyWorkspaceScreen {
        StudyWorkspaceScreen::new(vec![
            self.flashcards_entry(layout),
            self.multiple_choice_entry(layout),
        ])
    }

    fn flashcards_entry(&self, layout: &StudySetLayout) -> ModeEntry {
        FlashcardDeck::from_markdown_file(&layout.flashcards_path())
            .map(|deck| {
                deck.is_empty()
                    .then(|| {
                        ModeEntry::unavailable(
                            StudyMode::Flashcards,
                            "No flashcards found in generated/flashcards/flashcards.md.",
                        )
                    })
                    .unwrap_or_else(|| {
                        ModeEntry::available(
                            StudyMode::Flashcards,
                            Box::new(FlashcardStudyScreen::new(deck)),
                        )
                    })
            })
            .unwrap_or_else(|error| {
                ModeEntry::unavailable(StudyMode::Flashcards, error.to_string())
            })
    }

    fn multiple_choice_entry(&self, layout: &StudySetLayout) -> ModeEntry {
        MultipleChoiceDeck::from_markdown_file(&layout.multiple_choice_path())
            .map(|deck| {
                deck.is_empty()
                    .then(|| {
                        ModeEntry::unavailable(
                            StudyMode::MultipleChoice,
                            "No multiple-choice questions parsed.",
                        )
                    })
                    .unwrap_or_else(|| {
                        ModeEntry::available(
                            StudyMode::MultipleChoice,
                            Box::new(MultipleChoiceScreen::new(deck)),
                        )
                    })
            })
            .unwrap_or_else(|error| {
                ModeEntry::unavailable(StudyMode::MultipleChoice, error.to_string())
            })
    }

    fn run_loop(
        &self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        screen: &mut dyn Screen,
    ) -> Result<()> {
        std::iter::repeat_with(|| ())
            .map(|_| self.tick(terminal, screen))
            .find(|result| !matches!(result, Ok(ScreenTransition::Stay)))
            .transpose()
            .map(|_| ())
    }

    fn tick(
        &self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        screen: &mut dyn Screen,
    ) -> Result<ScreenTransition> {
        terminal.draw(|frame| screen.render(frame, frame.area()))?;
        event::poll(Duration::from_millis(250))?
            .then(event::read)
            .transpose()?
            .map(|event| screen.handle_event(event))
            .transpose()
            .map(|transition| transition.unwrap_or(ScreenTransition::Stay))
    }
}
