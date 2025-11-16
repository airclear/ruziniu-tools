# Project Overview

This is a desktop application built with [Tauri](https://tauri.app/) and [Vue.js 3](https://vuejs.org/). The frontend is built with Vue.js and the backend is a Rust application that is exposed to the frontend.

The application is a simple "Hello, World!" style application that takes a name as input and displays a greeting from the Rust backend.

# Building and Running

To build and run this project, you will need to have [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/) installed.

## Development

To run the application in development mode, use the following command:

```bash
npm run tauri dev
```

This will start the Vite development server for the frontend and build and run the Tauri application.

## Production

To build the application for production, use the following command:

```bash
npm run tauri build
```

This will create a production-ready executable in the `src-tauri/target/release` directory.

# Development Conventions

The project uses the standard conventions for Tauri and Vue.js projects.

## Frontend

The frontend code is located in the `src` directory. The main component is `src/App.vue`. The project uses Vite for the development server and build tool.

## Backend

The backend code is located in the `src-tauri` directory. The main entry point for the backend is `src-tauri/src/main.rs`. The core logic is in `src-tauri/src/lib.rs`. The backend exposes a `greet` command to the frontend.
