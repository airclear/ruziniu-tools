# Project Context

## Purpose
This is a desktop application built with [Tauri](https://tauri.app/) and [Vue.js 3](https://vuejs.org/). The frontend is built with Vue.js and the backend is a Rust application that is exposed to the frontend.

The application is a simple "Hello, World!" style application that takes a name as input and displays a greeting from the Rust backend.

## Tech Stack
- [Vue.js 3](https://vuejs.org/)
- [Tauri](https://tauri.app/)
- [Vite](https://vitejs.dev/)
- [Rust](https://www.rust-lang.org/)

## Project Conventions

### Code Style
The project uses the standard conventions for Tauri and Vue.js projects. The frontend code is located in the `src` directory. The main component is `src/App.vue`. The project uses Vite for the development server and build tool.

### Architecture Patterns
The project follows a simple frontend-backend architecture. The frontend is a Vue.js 3 application that communicates with the backend via Tauri's `invoke` API. The backend is a Rust application that exposes commands to the frontend.

### Testing Strategy
[TODO: Add testing strategy]

### Git Workflow
[TODO: Add git workflow]

## Domain Context
The application is a simple demonstration of how to build a desktop application with Tauri and Vue.js.

## Important Constraints
The project requires Node.js and Rust to be installed.

## External Dependencies
There are no external dependencies other than the ones listed in the `package.json` and `Cargo.toml` files.
