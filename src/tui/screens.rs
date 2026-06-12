use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::{
    tui::{
        FlashcardDeck, FlashcardStudySession, MultipleChoiceDeck, MultipleChoiceSession, StudyMode,
        render_helpers::*,
        helpers::*
    },
    Result,
};


/// Result of handling screen input.
pub enum ScreenTransition {
    /// Keep displaying the current screen.
    Stay,
    /// Exit the terminal application.
    Quit,
}

/// A renderable and interactive TUI screen.
pub trait Screen {
    /// Renders the screen into the provided frame.
    fn render(&self, frame: &mut Frame, area: Rect);

    /// Handles an input event.
    fn handle_event(&mut self, event: crossterm::event::Event) -> Result<ScreenTransition>;
}

/// Root TUI screen that owns study modes and the command palette.
pub struct StudyWorkspaceScreen {
    modes: Vec<ModeEntry>,
    active: usize,
    palette: CommandPalette,
}

impl StudyWorkspaceScreen {
    /// Creates a workspace screen from available mode entries.
    pub fn new(modes: Vec<ModeEntry>) -> Self {
        Self {
            modes,
            active: 0,
            palette: CommandPalette::new(),
        }
    }

    /// Creates a dashboard-only workspace.
    pub fn dashboard(message: impl Into<String>) -> Self {
        Self::new(vec![ModeEntry::available(
            StudyMode::Flashcards,
            Box::new(DashboardScreen::new(message)),
        )])
    }
}

impl Screen for StudyWorkspaceScreen {
    fn render(&self, frame: &mut Frame, area: Rect) {
        self.active_screen().render(frame, area);
        self.palette.is_open().then(|| {
            self.palette
                .render(frame, centered_rect(62, 52, area), &self.modes)
        });
    }

    fn handle_event(&mut self, event: crossterm::event::Event) -> Result<ScreenTransition> {
        Ok(event
            .as_key_press_event()
            .map(|key| self.handle_key(key))
            .unwrap_or(ScreenTransition::Stay))
    }
}

impl StudyWorkspaceScreen {
    fn active_screen(&self) -> &dyn Screen {
        self.modes[self.active].screen.as_ref()
    }

    fn active_screen_mut(&mut self) -> &mut dyn Screen {
        self.modes[self.active].screen.as_mut()
    }

    fn handle_key(&mut self, key: &crossterm::event::KeyEvent) -> ScreenTransition {
        self.palette
            .is_open()
            .then(|| self.handle_palette_key(key))
            .unwrap_or_else(|| self.handle_workspace_key(key))
    }

    fn handle_workspace_key(&mut self, key: &crossterm::event::KeyEvent) -> ScreenTransition {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('p') | KeyCode::Char(':') => {
                self.palette.open(self.active);
                ScreenTransition::Stay
            }
            _ => self
                .active_screen_mut()
                .handle_event(crossterm::event::Event::Key(*key))
                .unwrap_or(ScreenTransition::Stay),
        }
    }

    fn handle_palette_key(&mut self, key: &crossterm::event::KeyEvent) -> ScreenTransition {
        self.palette
            .handle_key(key, self.modes.len())
            .map_or(ScreenTransition::Stay, |selected| {
                self.active = selected;
                ScreenTransition::Stay
            })
    }
}

/// One selectable study mode entry.
pub struct ModeEntry {
    mode: StudyMode,
    screen: Box<dyn Screen>,
    available: bool,
    status: String,
}

impl ModeEntry {
    /// Creates an available study mode.
    pub fn available(mode: StudyMode, screen: Box<dyn Screen>) -> Self {
        Self {
            mode,
            screen,
            available: true,
            status: "Ready".to_owned(),
        }
    }

    /// Creates an unavailable study mode with a reason.
    pub fn unavailable(mode: StudyMode, reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            mode,
            screen: Box::new(DashboardScreen::new(reason.clone())),
            available: false,
            status: reason,
        }
    }
}

struct CommandPalette {
    open: bool,
    selected: usize,
}

impl CommandPalette {
    fn new() -> Self {
        Self {
            open: false,
            selected: 0,
        }
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn open(&mut self, active: usize) {
        self.open = true;
        self.selected = active;
    }

    fn render(&self, frame: &mut Frame, area: Rect, modes: &[ModeEntry]) {
        frame.render_widget(Clear, area);
        let items = modes
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let marker = (index == self.selected).then_some("> ").unwrap_or("  ");
                let status = entry.available.then_some("ready").unwrap_or("missing");
                ListItem::new(format!(
                    "{marker}{}  [{}]  {}",
                    entry.mode.label(),
                    status,
                    entry.status
                ))
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            List::new(items).block(
                Block::default()
                    .title("Command Palette | up/down, enter, 1-4, esc")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            ),
            area,
        );
    }

    fn handle_key(&mut self, key: &crossterm::event::KeyEvent, len: usize) -> Option<usize> {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Esc | KeyCode::Char('p') | KeyCode::Char(':') => {
                self.open = false;
                None
            }
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
                None
            }
            KeyCode::Down => {
                self.selected = (self.selected + 1).min(len.saturating_sub(1));
                None
            }
            KeyCode::Enter => {
                self.open = false;
                Some(self.selected)
            }
            KeyCode::Char(value) => value
                .to_digit(10)
                .and_then(|digit| usize::try_from(digit).ok())
                .and_then(|digit| digit.checked_sub(1))
                .filter(|index| *index < len)
                .map(|index| {
                    self.open = false;
                    index
                }),
            _ => None,
        }
    }
}

/// Initial dashboard screen for unavailable study modes.
pub struct DashboardScreen {
    message: String,
}

impl DashboardScreen {
    /// Creates a dashboard with a status message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Screen for DashboardScreen {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title("Localearn").borders(Borders::ALL);
        let text = Paragraph::new(format!(
            "{}\n\nPress p for modes or q to quit.",
            self.message
        ))
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        frame.render_widget(text, area);
    }

    fn handle_event(&mut self, event: crossterm::event::Event) -> Result<ScreenTransition> {
        Ok(event
            .as_key_press_event()
            .and_then(QuitKeyBinding::transition)
            .unwrap_or(ScreenTransition::Stay))
    }
}

/// Flashcard study screen with Anki-like reveal and grading.
pub struct FlashcardStudyScreen {
    session: FlashcardStudySession,
}

impl FlashcardStudyScreen {
    /// Creates a flashcard study screen from a loaded deck.
    pub fn new(deck: FlashcardDeck) -> Self {
        Self {
            session: FlashcardStudySession::new(deck),
        }
    }
}

impl Screen for FlashcardStudyScreen {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let [header, progress, card, footer] = study_layout(area);
        render_header(frame, header, "Flashcards", self.stats_text());
        render_progress(frame, progress, self.progress_ratio());
        render_box(
            frame,
            card,
            &self.card_title(),
            &self.card_text(),
            Color::Cyan,
        );
        render_footer(frame, footer, self.controls());
    }

    fn handle_event(&mut self, event: crossterm::event::Event) -> Result<ScreenTransition> {
        Ok(event
            .as_key_press_event()
            .map(|key| FlashcardKeyBinding::from_key(key).apply(&mut self.session))
            .unwrap_or(ScreenTransition::Stay))
    }
}

impl FlashcardStudyScreen {
    fn stats_text(&self) -> String {
        let stats = self.session.stats();
        format!(
            "Reviewed {} | Known {} | Missed {} | Remaining {}",
            stats.reviewed, stats.known, stats.missed, stats.remaining
        )
    }

    pub fn progress_ratio(&self) -> f64 {
        let stats = self.session.stats();
        progress_ratio(stats.total, stats.remaining)
    }

    fn card_title(&self) -> String {
        self.session
            .current()
            .map(|card| card.title().to_owned())
            .unwrap_or_else(|| "Session complete".to_owned())
    }

    fn card_text(&self) -> String {
        self.session
            .current()
            .map(|card| {
                self.session
                    .showing_answer()
                    .then(|| format!("{}\n\n---\n\n{}", card.front(), card.back()))
                    .unwrap_or_else(|| card.front().to_owned())
            })
            .unwrap_or_else(|| "All cards are currently marked known.".to_owned())
    }

    fn controls(&self) -> &'static str {
        self.session
            .showing_answer()
            .then_some("1 Again | 2 Good | s Skip | p Palette | q Quit")
            .unwrap_or("Space/Enter Reveal | s Skip | p Palette | q Quit")
    }
}

/// Multiple-choice study screen.
pub struct MultipleChoiceScreen {
    session: MultipleChoiceSession,
}

impl MultipleChoiceScreen {
    /// Creates a multiple-choice screen.
    pub fn new(deck: MultipleChoiceDeck) -> Self {
        Self {
            session: MultipleChoiceSession::new(deck),
        }
    }
}

impl Screen for MultipleChoiceScreen {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let [header, progress, card, footer] = study_layout(area);
        let stats = self.session.stats();
        render_header(
            frame,
            header,
            "Multiple Choice",
            format!("Answered {} | Correct {}", stats.answered, stats.correct),
        );
        render_progress(
            frame,
            progress,
            progress_ratio(stats.total, stats.total.saturating_sub(stats.index)),
        );
        render_box(frame, card, &self.title(), &self.body(), Color::Magenta);
        render_footer(frame, footer, "1-4 Choose | n Next | p Palette | q Quit");
    }

    fn handle_event(&mut self, event: crossterm::event::Event) -> Result<ScreenTransition> {
        Ok(event
            .as_key_press_event()
            .map(|key| MultipleChoiceKeyBinding::from_key(key).apply(&mut self.session))
            .unwrap_or(ScreenTransition::Stay))
    }
}

impl MultipleChoiceScreen {
    fn title(&self) -> String {
        self.session
            .current()
            .map(|question| question.title().to_owned())
            .unwrap_or_else(|| "Multiple choice complete".to_owned())
    }

    fn body(&self) -> String {
        self.session
            .current()
            .map(|question| {
                let options = question
                    .options()
                    .iter()
                    .enumerate()
                    .map(|(index, option)| {
                        let marker = self
                            .session
                            .selected()
                            .filter(|selected| *selected == index)
                            .map(|_| "*")
                            .unwrap_or(" ");
                        format!("{marker} {}. {option}", index + 1)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let feedback = self
                    .session
                    .selected()
                    .map(|_| {
                        format!(
                            "\n\nAnswer: {}\n{}",
                            question.answer(),
                            question.explanation()
                        )
                    })
                    .unwrap_or_default();
                format!("{}\n\n{}{}", question.prompt(), options, feedback)
            })
            .unwrap_or_else(|| "All questions complete.".to_owned())
    }
}

enum FlashcardKeyBinding {
    Reveal,
    Known,
    Missed,
    Skip,
    Quit,
    Ignore,
}

impl FlashcardKeyBinding {
    fn from_key(key: &crossterm::event::KeyEvent) -> Self {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Self::Quit,
            KeyCode::Char(' ') | KeyCode::Enter => Self::Reveal,
            KeyCode::Char('1') | KeyCode::Char('a') => Self::Missed,
            KeyCode::Char('2') | KeyCode::Char('g') => Self::Known,
            KeyCode::Char('s') | KeyCode::Right => Self::Skip,
            _ => Self::Ignore,
        }
    }

    fn apply(self, session: &mut FlashcardStudySession) -> ScreenTransition {
        match self {
            Self::Reveal => {
                session.reveal();
                ScreenTransition::Stay
            }
            Self::Known => {
                session.showing_answer().then(|| session.mark_known());
                ScreenTransition::Stay
            }
            Self::Missed => {
                session.showing_answer().then(|| session.mark_missed());
                ScreenTransition::Stay
            }
            Self::Skip => {
                session.skip();
                ScreenTransition::Stay
            }
            Self::Quit => ScreenTransition::Quit,
            Self::Ignore => ScreenTransition::Stay,
        }
    }
}

enum MultipleChoiceKeyBinding {
    Select(usize),
    Next,
    Quit,
    Ignore,
}

impl MultipleChoiceKeyBinding {
    fn from_key(key: &crossterm::event::KeyEvent) -> Self {
        use crossterm::event::KeyCode;

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Self::Quit,
            KeyCode::Char('n') | KeyCode::Right | KeyCode::Enter => Self::Next,
            KeyCode::Char('1') => Self::Select(0),
            KeyCode::Char('2') => Self::Select(1),
            KeyCode::Char('3') => Self::Select(2),
            KeyCode::Char('4') => Self::Select(3),
            _ => Self::Ignore,
        }
    }

    fn apply(self, session: &mut MultipleChoiceSession) -> ScreenTransition {
        match self {
            Self::Select(index) => {
                session.select(index);
                ScreenTransition::Stay
            }
            Self::Next => {
                session.next();
                ScreenTransition::Stay
            }
            Self::Quit => ScreenTransition::Quit,
            Self::Ignore => ScreenTransition::Stay,
        }
    }
}

struct QuitKeyBinding;

impl QuitKeyBinding {
    fn transition(key: &crossterm::event::KeyEvent) -> Option<ScreenTransition> {
        use crossterm::event::KeyCode;

        matches!(key.code, KeyCode::Char('q') | KeyCode::Esc).then_some(ScreenTransition::Quit)
    }
}

trait KeyPressEvent {
    fn as_key_press_event(&self) -> Option<&crossterm::event::KeyEvent>;
}

impl KeyPressEvent for crossterm::event::Event {
    fn as_key_press_event(&self) -> Option<&crossterm::event::KeyEvent> {
        use crossterm::event::{Event, KeyEventKind};

        let Event::Key(key) = self else {
            return None;
        };

        (key.kind == KeyEventKind::Press).then_some(key)
    }
}

