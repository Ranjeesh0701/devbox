# Devbox

**Let AI build. Stay in control.**

Devbox is a development process manager designed for the AI-assisted coding era. It aims to make it easier to start, manage, and observe the processes behind your applications — from frontend servers and backend APIs to workers and test runners.

AI coding agents can write code and execute commands. Devbox focuses on the next part: keeping development processes manageable and making their state visible to developers.

> **Project status:** Under active development. Features and commands may change as the project evolves.

## The problem

Modern applications often require several processes running at the same time:

- A frontend development server
- A backend API
- Background workers
- Databases and supporting services
- Tests, builds, and other development tasks

As AI coding agents become part of everyday development, managing these processes through scattered terminal sessions can become harder to follow. A process may exit unexpectedly, a server may fail to start, or its output may be buried in terminal history.

Devbox aims to provide a consistent way to manage these processes and a clear place for developers to understand what's happening.

## Vision

Devbox is being built around three ideas:

- **Agent-friendly:** Provide a predictable interface for tools and coding agents to manage development processes.
- **Observable:** Make process state and output easier to inspect.
- **Human-controlled:** Give developers a clear view of the processes running their applications.

The long-term goal is a workflow where an AI agent can work on an application while the developer retains visibility and control over its runtime.

## How it fits together

```text
AI coding agent
      |
      | Process commands
      v
   Devbox
      |
      +---- Frontend
      |
      +---- Backend
      |
      +---- Workers
      |
      v
Process state and output
      |
      v
Developer dashboard
```

This represents the intended direction of the project. AI-agent integrations and the complete visual workflow are not implied to be available today.

## Features

Devbox is being developed toward the following capabilities:

- **Process lifecycle management** — launch and manage development processes.
- **Session tracking** — associate processes with identifiable sessions.
- **Process status** — distinguish running processes from completed or inactive ones.
- **Exit information** — make process termination and exit codes available.
- **Persistent session metadata** — retain session information between CLI invocations.
- **Visual interface** — provide a desktop view of development processes.
- **AI-agent integration** — build toward structured process controls that coding agents can use reliably.

Refer to the current implementation and CLI help for commands that are actually supported.

## Project structure

```text
devbox/
├── src/            # Rust application and process management
├── desktop/        # Desktop interface
├── website/        # Public landing page
├── Cargo.toml      # Rust dependencies and configuration
└── README.md
```

The desktop application and website are separate frontend projects.

## Getting started

### Prerequisites

Install the tools required by the parts of the project you want to work on:

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [Bun](https://bun.sh/) for the JavaScript projects
- [Node.js](https://nodejs.org/) if required by your local tooling
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) to build the desktop application

### Run the Rust application

From the repository root:

```bash
cargo run
```

Use the command syntax and actions implemented in the current source code. The CLI interface is still evolving.

### Run the website

```bash
cd website
bun install
bun run dev
```

Create a production build:

```bash
bun run build
```

### Run the desktop interface

```bash
cd desktop
bun install
bun run tauri dev
```

The desktop command assumes the Tauri CLI and required native build dependencies are configured.

## Development principles

Devbox aims to follow a few principles as it evolves:

1. **Reliable process control over clever automation.**
2. **Structured state over parsing terminal output.**
3. **Clear failure information over silent errors.**
4. **Visibility for developers and predictable behavior for tools.**
5. **Small, composable features over unnecessary complexity.**

## Contributing

Devbox is evolving incrementally. Contributions, bug reports, and ideas around process management, developer experience, and AI-assisted workflows are welcome.

Before contributing:

1. Check the current implementation and open issues.
2. Keep changes focused and easy to review.
3. Include clear reproduction steps for bugs.
4. Avoid claiming integrations or functionality that are not implemented.

## License

The project license has not yet been specified. Add a `LICENSE` file before distributing Devbox under an open-source license.

---

**Devbox — a clearer view of the processes behind your code.**
