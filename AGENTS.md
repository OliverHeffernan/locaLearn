# Localearn Agent Instructions

Agents working in this repository may:

- Add Rust modules that extend existing traits.
- Add tests and documentation for new study workflows.
- Keep `src/main.rs` small and delegate behavior into library modules.
- Prefer new provider, artifact, command, and screen implementations over rewriting existing ones.

Agents must not:

- Remove user study resources.
- Write outside the active workspace unless explicitly requested.
- Couple generation logic directly to a single provider implementation.
- Add hard-coded study-set paths outside the layout abstractions.
