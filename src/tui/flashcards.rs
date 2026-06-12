use std::{collections::VecDeque, fs, path::Path};

use crate::Result;

/// One generated flashcard with a front prompt and back answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flashcard {
    title: String,
    front: String,
    back: String,
}

impl Flashcard {
    /// Creates a flashcard.
    pub fn new(
        title: impl Into<String>,
        front: impl Into<String>,
        back: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            front: front.into(),
            back: back.into(),
        }
    }

    /// Returns the card title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the prompt side.
    pub fn front(&self) -> &str {
        &self.front
    }

    /// Returns the answer side.
    pub fn back(&self) -> &str {
        &self.back
    }
}

/// A collection of flashcards loaded from generated content.
#[derive(Debug, Clone)]
pub struct FlashcardDeck {
    cards: Vec<Flashcard>,
}

impl FlashcardDeck {
    /// Loads a deck from a generated Markdown file.
    pub fn from_markdown_file(path: &Path) -> Result<Self> {
        fs::read_to_string(path)
            .map(|content| FlashcardParser::default().parse(&content))
            .map(|cards| Self { cards })
            .map_err(Into::into)
    }

    /// Returns cards in the deck.
    pub fn cards(&self) -> &[Flashcard] {
        &self.cards
    }

    /// Returns the number of cards in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns whether the deck contains no cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

/// Parses generated Markdown flashcards.
#[derive(Default)]
pub struct FlashcardParser;

impl FlashcardParser {
    /// Parses flashcards from Markdown using front/back field markers.
    pub fn parse(&self, content: &str) -> Vec<Flashcard> {
        content
            .split("***")
            .filter_map(CardBlock::from_markdown)
            .collect()
    }
}

struct CardBlock;

impl CardBlock {
    fn from_markdown(block: &str) -> Option<Flashcard> {
        let title = block
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("### Card"))
            .unwrap_or("Flashcard");

        block
            .split_once("**Front:**")
            .and_then(|(_, rest)| rest.split_once("**Back:**"))
            .map(|(front, back)| {
                Flashcard::new(
                    title.trim_start_matches('#').trim(),
                    front.trim(),
                    back.trim(),
                )
            })
            .filter(|card| !card.front().is_empty() && !card.back().is_empty())
    }
}

/// Tracks one flashcard study session.
#[derive(Debug, Clone)]
pub struct FlashcardStudySession {
    queue: VecDeque<Flashcard>,
    current: Option<Flashcard>,
    reviewed: usize,
    known: usize,
    missed: usize,
    showing_answer: bool,
    total: usize,
}

impl FlashcardStudySession {
    /// Starts a session from a deck.
    pub fn new(deck: FlashcardDeck) -> Self {
        let mut queue = deck.cards().iter().cloned().collect::<VecDeque<_>>();
        let current = queue.pop_front();
        Self {
            queue,
            current,
            reviewed: 0,
            known: 0,
            missed: 0,
            showing_answer: false,
            total: deck.len(),
        }
    }

    /// Returns the active card.
    pub fn current(&self) -> Option<&Flashcard> {
        self.current.as_ref()
    }

    /// Returns whether the answer side is currently visible.
    pub fn showing_answer(&self) -> bool {
        self.showing_answer
    }

    /// Reveals the answer for the current card.
    pub fn reveal(&mut self) {
        self.showing_answer = true;
    }

    /// Marks the current card as known and advances.
    pub fn mark_known(&mut self) {
        self.known += self.current.iter().count();
        self.reviewed += self.current.iter().count();
        self.advance();
    }

    /// Marks the current card as missed, requeues it, and advances.
    pub fn mark_missed(&mut self) {
        self.current
            .take()
            .map(|card| {
                self.missed += 1;
                self.reviewed += 1;
                self.queue.push_back(card);
            })
            .unwrap_or_default();
        self.current = self.queue.pop_front();
        self.showing_answer = false;
    }

    /// Skips the current card by moving it to the back of the queue.
    pub fn skip(&mut self) {
        self.current
            .take()
            .map(|card| self.queue.push_back(card))
            .unwrap_or_default();
        self.current = self.queue.pop_front();
        self.showing_answer = false;
    }

    /// Returns progress statistics for display.
    pub fn stats(&self) -> FlashcardStudyStats {
        FlashcardStudyStats {
            total: self.total,
            remaining: self.queue.len() + self.current.iter().count(),
            reviewed: self.reviewed,
            known: self.known,
            missed: self.missed,
        }
    }

    fn advance(&mut self) {
        self.current = self.queue.pop_front();
        self.showing_answer = false;
    }
}

/// Summary of a flashcard study session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlashcardStudyStats {
    /// Original number of cards loaded.
    pub total: usize,
    /// Cards still in the active queue.
    pub remaining: usize,
    /// Number of grading actions made.
    pub reviewed: usize,
    /// Cards marked as known.
    pub known: usize,
    /// Cards marked as missed.
    pub missed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_generated_markdown_flashcards() {
        let content = r#"
### Flashcards

***

### Card 1: Basics
**Front:** What is Rust ownership?
**Back:** A compile-time memory management model.

***

### Card 2: Borrowing
**Front:** What does `&T` mean?
**Back:** An immutable reference.
"#;

        let cards = FlashcardParser::default().parse(content);

        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].title(), "Card 1: Basics");
        assert_eq!(cards[0].front(), "What is Rust ownership?");
        assert_eq!(cards[0].back(), "A compile-time memory management model.");
    }

    #[test]
    fn missed_cards_return_to_queue() {
        let deck = FlashcardDeck {
            cards: vec![
                Flashcard::new("One", "front 1", "back 1"),
                Flashcard::new("Two", "front 2", "back 2"),
            ],
        };
        let mut session = FlashcardStudySession::new(deck);

        session.reveal();
        session.mark_missed();

        let stats = session.stats();
        assert_eq!(stats.missed, 1);
        assert_eq!(stats.remaining, 2);
        assert_eq!(session.current().map(Flashcard::title), Some("Two"));
    }
}
