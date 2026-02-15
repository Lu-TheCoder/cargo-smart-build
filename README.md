# cargo-smart-build

**Interactive cargo subcommand to easily select, configure, and build workspace crates with remembered preferences.**

![ezgif-64992ecde83193ff](https://github.com/user-attachments/assets/9a8a1761-3f44-4c82-badf-67f99fda50f2)


### The Problem
Working in a multi-crate workspace often means typing repetitive, long commands: 
`cargo build -p <package> --features <feature1,feature2> --release`

### The Solution
**`cargo smart-build` remembers your choices.**

Simply run `cargo smart-build` once, select your package and features interactively, and build. Next time? Just run it again—it remembers everything.

## Features

-   **Interactive Workspace Selection**: Automatically detects workspace members and lets you select the package to build from a list.
-   **Execution Modes**:
    -   `cargo smart-build` (or `build`): Builds the selected crate.
    -   `cargo smart-build run`: Builds and runs the binary.
    -   `cargo smart-build test`: Runs tests for the selected crate/features.
-   **Target Selector**: 
    -   Auto-detects installed Rust targets via `rustup`.
    -   Allows selecting a specific target architecture (e.g., `wasm32-unknown-unknown`) or using the default native target.
    -   Supports installing new targets directly from the UI.
    -   **Seamless Cross-Compilation**: Automatically switches to use [`cross`](https://github.com/cross-rs/cross) when a non-native target is selected (requires Docker).
    -   **Auto-Install**: Prompts to install `cross` if it's missing.
-   **Smart Defaults**: Remembers your last selected package, build mode, target, and features. These are pre-selected (and highlighted in green) the next time you run the tool.
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
2.  Run one of the following commands:

    ```bash
    # Build only
    cargo smart-build 
    
    # Run binary
    cargo smart-build run
    
    # Run tests
    cargo smart-build test
    ```

3.  Follow the interactive prompts:
    -   **Select Package**: Choose the crate you want to build/run/test.
    -   **Select Build Mode**: Choose `Debug` or `Release`.
    -   **Select Features**: Toggle features on/off (Space to select, Enter to confirm).

4.  The tool will execute the appropriate cargo command (`build`, `run`, `test`).

## Configuration

Your preferences are stored in `.smart-build/config.json` in your workspace root. You can add `.smart-build/` to your `.gitignore` if you don't want to commit personal build preferences.

## License

This project is licensed under the [MIT License](LICENSE).
