# NWEN241 Agent Instructions

Agents may:

- Read files inside `resources/`.
- Generate study artifacts into `generated/`.
- Update `study.toml` when metadata changes.
- Create flashcards, multiple-choice questions, fill-in-the-blanks, and practice tests.

Agents must not:

- Delete source files in `resources/`.
- Write outside this study set directory.
- Send content to a provider other than the configured provider.
- Replace generated artifacts without preserving the user's source resources.
