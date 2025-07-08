# Line Cook

An MCP to help with executing pieces-wise refactors and generations. I believe that there is an increase in LLM effectiveness when the information needed for a task to be completed is minimized. This is an effort to provide guidance with minimization.

## Features

- [ ] Scrap ToDo list, to be created for step expansion and to be deleted when completed. Assists with larger tasks, aimed to isolate efforts for minimized context.
- [ ] Dependency injection generalization from multiple specific code snippets. Help with DRY concepts.

## Tech Stack

- [nix flake](https://serokell.io/blog/practical-nix-flakes#basic-flake-structure), for development environment and package creation
- Python, for implementation
- [uv](https://docs.astral.sh/uv/), for Python project management
- [FastMCP](https://gofastmcp.com/python-sdk/)
