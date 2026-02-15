# cargo-smart-build

**Interactive cargo subcommand to easily select, configure, and build workspace crates with remembered preferences.**

`cargo-smart-build` streamlines your workflow in multi-crate workspaces.
Instead of typing long `cargo build -p <package> --features <...>` commands, simply run `cargo smart-build` and interactively select what you want to build. The tool remembers your choices, making subsequent builds instant.

## Features

-   **Interactive Workspace Selection**: Automatically detects workspace members and lets you select the package to build from a list.
-   **Smart Defaults**: Remembers your last selected package, build mode, and features. These are pre-selected (and highlighted in green) the next time you run the tool.
-   **Build Mode Toggle**: Easily switch between **Debug** and **Release** builds without manual flags.
-   **Feature Management**: 
    -   Lists all available features for the selected package (excluding `default`).
    -   Allows multi-selection of features.
    -   Remembers enabled features for each package.
-   **Persistence**: Saves your preferences locally in a `.smart-build/config.json` file, so your environment is always ready.

## Installation

Install via cargo:

```bash
cargo install cargo-smart-build
```

## Usage

1.  Navigate to the root of your Cargo workspace.
2.  Run the command:

    ```bash
    cargo smart-build
    ```

3.  Follow the interactive prompts:
    -   **Select Package**: Choose the crate you want to build.
    -   **Select Build Mode**: Choose `Debug` or `Release`.
    -   **Select Features**: Toggle features on/off (Space to select, Enter to confirm).

4.  The tool will construct and run the appropriate `cargo build` command.

## Configuration

Your preferences are stored in `.smart-build/config.json` in your workspace root. You can add `.smart-build/` to your `.gitignore` if you don't want to commit personal build preferences.

## License

This project is licensed under the [MIT License](LICENSE).
