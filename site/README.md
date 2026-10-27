# Artificial Culture — project website

An editorial, bilingual presentation of the Artificial Culture cognition layer for Gala of the Hidden. The site is written in Portuguese and English and is intentionally explicit about the difference between the active C++20/Flecs foundation, the former Rust/Bevy work, and future acceptance criteria.

## Stack

- React 19.3
- TypeScript 7
- Vite 8
- CSS, with no UI framework or component-library dependency

## Run locally

From this directory:

```powershell
npm install
npm run dev
```

Create a production build with:

```powershell
npm run build
```

Vite writes the static site to `dist/`. The banner in `public/assets/` is a copy of the existing GitHub banner at the project root, so the site can be built and hosted independently.

## Content references

The narrative and technical descriptions are based on `../README.md`, `../Docs/ARCHITECTURE.md`, `../Docs/CULTURAL_SCENARIOS.md`, `../Docs/Research/README.md`, `../include/artificial_culture/cognition.hpp`, and `../src/`. The interactive action walkthrough follows the example in `../src/main.cpp`. The video embed is the historical Bevy demonstration supplied by the project owner; it is clearly labeled as a legacy version.

The current core remains a partial migration, not a complete port of the former simulation. The page does not claim cultural realism or a finished GotH server integration.
