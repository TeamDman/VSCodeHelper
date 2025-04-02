# VSCodeHelper

A Rust library for interacting with Visual Studio Code's configuration and state files.

## Overview

VSCodeHelper provides a convenient way to access and manipulate various VS Code configuration files, including:

- `storage.json` - Contains information about open windows, workspaces, themes, and more
- `state.vscdb` - SQLite database containing recently opened paths and other state information
- Workspace configuration files

## Features

- Load and parse VS Code's `storage.json` file
- Access recently opened folders and workspaces
- Query the `state.vscdb` SQLite database
- Retrieve workspace configurations
- Strong typing for VS Code's configuration structures

## Examples

### List Recently Opened Paths

```rust
use vscodehelper::state_vscdb::keys::history_recently_opened_paths_list::HistoryRecentlyOpenedPathsListKey;
use vscodehelper::state_vscdb::state_vscdb::StateVscdb;

fn main() -> eyre::Result<()> {
    let mut state_vscdb = StateVscdb::try_default()?;
    let recently_opened = state_vscdb.read::<HistoryRecentlyOpenedPathsListKey>()?;
    
    println!("Recently opened paths:");
    for entry in recently_opened.entries.iter() {
        println!("  - {:?}", entry);
    }

    Ok(())
}
```

### Get Currently Opened Windows

```rust
use vscodehelper::storage_json::storage_json::VSCodeStorageJson;
use vscodehelper::storage_json::window::Window;

fn main() -> eyre::Result<()> {
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    
    for window in &storage_json.windows_state.opened_windows {
        match window {
            Window::FolderWindow { folder, .. } => {
                println!("{}", folder.as_path()?.display());
            }
            Window::WorkspaceWindow { workspace_identifier, .. } => {
                let workspace_json = workspace_identifier.read()?;
                for folder in workspace_json.folders {
                    println!("{}", folder.path.display());
                }
            }
        }
    }
    
    Ok(())
}
```

### Get Current Theme

```rust
use vscodehelper::storage_json::storage_json::VSCodeStorageJson;

fn main() -> eyre::Result<()> {
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    println!("The active theme is {}", storage_json.theme);
    Ok(())
}
```

## Project Structure

- `src/storage_json/` - Types and functions for working with VS Code's storage.json
- `src/state_vscdb/` - Types and functions for working with VS Code's state.vscdb
- `src/workspace_json/` - Types for working with VS Code workspace files
- `vscodehelper-macros/` - Procedural macros for generating common trait implementations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vscodehelper = {git="https://github.com/TeamDman/VSCodeHelper"}
```

## Requirements

- Rust 2024 Edition
- VS Code installed on your system
- Probably only works on Windows lol