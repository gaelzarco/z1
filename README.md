# Portfolio (Perseus)

A small portfolio built using Perseus.

## Features
- Dynamic project pages at `/project/<slug>` (e.g., `/project/rgx`, `/project/space`, `/project/moxie`)

## Structure
- `src/templates/index.rs` — Home page
- `src/templates/project.rs` — Dynamic route `/project/<project_name>`
- `src/scripts/main.js` — Exports `fadeContent()`
- `static/styles.css`
- `static/projects.json` — Project data by slug
- `static/assets/` — Images, icons
- `static/fonts/` — Custom fonts

## Routing & Data
- Template path: `project/<project_name>` renders at `/project/<slug>`
- Prebuilt slugs via `build_paths_fn`: `rgx`, `space`, `moxie`
- `build_state_fn` (or `request_state_fn`) reads `static/projects.json` and selects the entry by slug
