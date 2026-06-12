use std::{fs, path::Path};

use crate::Result;

/// Study modes available in the terminal application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyMode {
    /// Generated flashcards.
    Flashcards,
    /// Generated multiple-choice questions.
    MultipleChoice,
    /// Generated fill-in-the-blanks exercises.
    FillBlanks,
    /// Generated practice test.
    PracticeTest,
}

impl StudyMode {
    /// Returns every mode in command-palette order.
    pub fn all() -> &'static [StudyMode] {
        &[
            StudyMode::Flashcards,
            StudyMode::MultipleChoice,
            StudyMode::FillBlanks,
            StudyMode::PracticeTest,
        ]
    }

    /// Returns the human-facing mode name.
    pub fn label(self) -> &'static str {
        match self {
            StudyMode::Flashcards => "Flashcards",
            StudyMode::MultipleChoice => "Multiple Choice",
            StudyMode::FillBlanks => "Fill in the Blanks",
            StudyMode::PracticeTest => "Practice Test",
        }
    }
}

/// One multiple-choice question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleChoiceQuestion {
    title: String,
    prompt: String,
    options: Vec<String>,
    answer: String,
    explanation: String,
}

impl MultipleChoiceQuestion {
    /// Returns the question title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the question prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns answer options.
    pub fn options(&self) -> &[String] {
        &self.options
    }

    /// Returns the correct answer text.
    pub fn answer(&self) -> &str {
        &self.answer
    }

    /// Returns the explanation.
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

/// Generated multiple-choice deck.
#[derive(Debug, Clone)]
pub struct MultipleChoiceDeck {
    questions: Vec<MultipleChoiceQuestion>,
}

impl MultipleChoiceDeck {
    /// Loads generated multiple-choice questions from Markdown.
    pub fn from_markdown_file(path: &Path) -> Result<Self> {
        fs::read_to_string(path)
            .map(|content| Self {
                questions: MultipleChoiceParser::default().parse(&content),
            })
            .map_err(Into::into)
    }

    /// Returns loaded questions.
    pub fn questions(&self) -> &[MultipleChoiceQuestion] {
        &self.questions
    }

    /// Returns whether no questions were loaded.
    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }
}

/// Parses multiple-choice Markdown.
#[derive(Default)]
pub struct MultipleChoiceParser;

impl MultipleChoiceParser {
    /// Parses common provider-generated multiple-choice Markdown.
    pub fn parse(&self, content: &str) -> Vec<MultipleChoiceQuestion> {
        split_markdown_items(content)
            .into_iter()
            .filter_map(parse_multiple_choice_block)
            .collect()
    }
}

/// Tracks a multiple-choice session.
#[derive(Debug, Clone)]
pub struct MultipleChoiceSession {
    questions: Vec<MultipleChoiceQuestion>,
    index: usize,
    selected: Option<usize>,
    submitted: bool,
    answered: usize,
    correct: usize,
}

impl MultipleChoiceSession {
    /// Starts a new multiple-choice session.
    pub fn new(deck: MultipleChoiceDeck) -> Self {
        Self {
            questions: deck.questions,
            index: 0,
            selected: None,
            submitted: false,
            answered: 0,
            correct: 0,
        }
    }

    /// Returns the current question.
    pub fn current(&self) -> Option<&MultipleChoiceQuestion> {
        self.questions.get(self.index)
    }

    /// Returns selected option index.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Selects an answer option.
    pub fn select(&mut self, option: usize) {
        self.selected = self.current().and(Some(option));
        self.submitted
            .then_some(())
            .unwrap_or_else(|| self.score_selection());
    }

    /// Advances to the next question.
    pub fn next(&mut self) {
        self.index = (self.index + 1).min(self.questions.len());
        self.selected = None;
        self.submitted = false;
    }

    /// Returns session stats.
    pub fn stats(&self) -> StudyStats {
        StudyStats {
            total: self.questions.len(),
            index: self.index,
            answered: self.answered,
            correct: self.correct,
        }
    }

    fn score_selection(&mut self) {
        let scored = self
            .selected
            .and_then(|selected| self.current().map(|question| (selected, question)))
            .and_then(|(selected, question)| {
                question
                    .options()
                    .get(selected)
                    .map(|option| option_matches_answer(option, question.answer()))
            });

        scored
            .map(|correct| {
                self.answered += 1;
                self.correct += usize::from(correct);
                self.submitted = true;
            })
            .unwrap_or_default();
    }
}

/// One fill-in-the-blank exercise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillBlankExercise {
    title: String,
    prompt: String,
    answer: String,
}

impl FillBlankExercise {
    /// Returns the exercise title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the exercise prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns the answer key or answer text.
    pub fn answer(&self) -> &str {
        &self.answer
    }
}

/// Generated fill-in-the-blanks deck.
#[derive(Debug, Clone)]
pub struct FillBlankDeck {
    exercises: Vec<FillBlankExercise>,
}

impl FillBlankDeck {
    /// Loads fill-in-the-blank exercises from Markdown.
    pub fn from_markdown_file(path: &Path) -> Result<Self> {
        fs::read_to_string(path)
            .map(|content| Self {
                exercises: FillBlankParser::default().parse(&content),
            })
            .map_err(Into::into)
    }

    /// Returns loaded exercises.
    pub fn exercises(&self) -> &[FillBlankExercise] {
        &self.exercises
    }

    /// Returns whether no exercises were loaded.
    pub fn is_empty(&self) -> bool {
        self.exercises.is_empty()
    }
}

/// Parses fill-in-the-blanks Markdown.
#[derive(Default)]
pub struct FillBlankParser;

impl FillBlankParser {
    /// Parses exercises and answer keys from Markdown.
    pub fn parse(&self, content: &str) -> Vec<FillBlankExercise> {
        let (questions, answer_key) = split_answer_key(content);
        let fallback_answer =
            answer_key.unwrap_or("Reveal the generated answer key in the source file.");
        split_markdown_items(questions)
            .into_iter()
            .filter(|item| item.contains("___") || item.to_lowercase().contains("blank"))
            .enumerate()
            .map(|(index, prompt)| FillBlankExercise {
                title: format!("Exercise {}", index + 1),
                prompt: prompt.trim().to_owned(),
                answer: fallback_answer.trim().to_owned(),
            })
            .collect()
    }
}

/// Tracks a fill-in-the-blanks session.
#[derive(Debug, Clone)]
pub struct FillBlankSession {
    exercises: Vec<FillBlankExercise>,
    index: usize,
    showing_answer: bool,
    known: usize,
    missed: usize,
}

impl FillBlankSession {
    /// Starts a fill-in-the-blanks session.
    pub fn new(deck: FillBlankDeck) -> Self {
        Self {
            exercises: deck.exercises,
            index: 0,
            showing_answer: false,
            known: 0,
            missed: 0,
        }
    }

    /// Returns the current exercise.
    pub fn current(&self) -> Option<&FillBlankExercise> {
        self.exercises.get(self.index)
    }

    /// Returns whether the answer is visible.
    pub fn showing_answer(&self) -> bool {
        self.showing_answer
    }

    /// Reveals the answer.
    pub fn reveal(&mut self) {
        self.showing_answer = true;
    }

    /// Marks the current exercise as known.
    pub fn mark_known(&mut self) {
        self.known += self.current().iter().count();
        self.advance();
    }

    /// Marks the current exercise as missed.
    pub fn mark_missed(&mut self) {
        self.missed += self.current().iter().count();
        self.advance();
    }

    /// Advances without grading.
    pub fn skip(&mut self) {
        self.advance();
    }

    /// Returns session stats.
    pub fn stats(&self) -> StudyStats {
        StudyStats {
            total: self.exercises.len(),
            index: self.index,
            answered: self.known + self.missed,
            correct: self.known,
        }
    }

    fn advance(&mut self) {
        self.index = (self.index + 1).min(self.exercises.len());
        self.showing_answer = false;
    }
}

/// Generated practice test document.
#[derive(Debug, Clone)]
pub struct PracticeTestDocument {
    content: String,
}

impl PracticeTestDocument {
    /// Loads a practice test from Markdown.
    pub fn from_markdown_file(path: &Path) -> Result<Self> {
        fs::read_to_string(path)
            .map(|content| Self { content })
            .map_err(Into::into)
    }

    /// Returns document content.
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Generic progress statistics for study methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudyStats {
    /// Total items in the study method.
    pub total: usize,
    /// Zero-based active item index.
    pub index: usize,
    /// Number of answered or graded items.
    pub answered: usize,
    /// Number of correct or known items.
    pub correct: usize,
}

fn split_markdown_items(content: &str) -> Vec<&str> {
    content
        .split("***")
        .flat_map(|chunk| chunk.split("\n### "))
        .map(str::trim)
        .filter(|chunk| !chunk.is_empty())
        .collect()
}

fn parse_multiple_choice_block(block: &str) -> Option<MultipleChoiceQuestion> {
    let lines = block.lines().map(str::trim).collect::<Vec<_>>();
    let title = lines
        .iter()
        .find(|line| line.to_lowercase().contains("question"))
        .copied()
        .unwrap_or("Question")
        .trim_start_matches('#')
        .trim()
        .to_owned();
    let options = lines
        .iter()
        .copied()
        .filter(|line| is_option_line(line))
        .map(clean_option)
        .collect::<Vec<_>>();
    let answer = find_prefixed_line(&lines, &["answer", "correct"]).unwrap_or_default();
    let explanation = find_prefixed_line(&lines, &["explanation"]).unwrap_or_default();
    let prompt = lines
        .iter()
        .copied()
        .filter(|line| !is_option_line(line))
        .filter(|line| !contains_label(line, &["answer", "correct", "explanation"]))
        .collect::<Vec<_>>()
        .join("\n");

    (!options.is_empty()).then_some(MultipleChoiceQuestion {
        title,
        prompt,
        options,
        answer,
        explanation,
    })
}

fn is_option_line(line: &str) -> bool {
    let trimmed = line.trim_start_matches(['-', '*', ' ']);
    matches!(
        trimmed.as_bytes(),
        [b'A'..=b'D', b')' | b'.' | b':', ..] | [b'a'..=b'd', b')' | b'.' | b':', ..]
    )
}

fn clean_option(line: &str) -> String {
    line.trim_start_matches(['-', '*', ' ']).trim().to_owned()
}

fn option_matches_answer(option: &str, answer: &str) -> bool {
    let option = normalize_answer(option);
    let answer = normalize_answer(answer);
    !answer.is_empty() && (option.contains(&answer) || answer.contains(&option))
}

fn normalize_answer(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn find_prefixed_line(lines: &[&str], labels: &[&str]) -> Option<String> {
    lines
        .iter()
        .copied()
        .find(|line| contains_label(line, labels))
        .map(|line| {
            line.split_once(':')
                .map(|(_, value)| value.trim().to_owned())
                .unwrap_or_else(|| line.to_owned())
        })
}

fn contains_label(line: &str, labels: &[&str]) -> bool {
    let lower = line.to_lowercase();
    labels.iter().any(|label| lower.contains(label))
}

fn split_answer_key(content: &str) -> (&str, Option<&str>) {
    content
        .split_once("Answer Key")
        .map(|(questions, answers)| (questions, Some(answers)))
        .unwrap_or((content, None))
}
