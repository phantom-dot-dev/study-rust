# Overview

This is a personalized rust study note.




## Vscode Setup Rust Analyzer:
Install the rust analyzer plugin in vscode. and link the analyzer executable using 

`"rust-analyzer.server.path": "/path-to-the-anal    ",` entry in vscode setting.json 

For termux setup (in Android), install `rust` and `rust-analyzer` from termux's packages using pkg. Then link the analyzer in vscode. Search the analyzer executabel by `which rust-analyzer` command.


### Init project with cargo:

`cargo new <project-name>` -> this will create the project directory with
- src: source directory with a `main.rs` file
- target:  
- Cargo.toml: Configuration file for dependencies and other (.toml: toms ovious minimal language)
- Cargo.lock: Not intendent for manual edit. It will be managed by Cargo for keeping different config information.
- `mod.rs` is a source code file used to define the contents and public interface of a module within the Rust language's module system. Optional science 2018, Intead, The compiler automatically looks for `module_name.rs` or a `module_name/` directory when mod `module_name` is declared in the parent file, making the code structure generally cleaner and easier to navigate.


### vscode code ruining shortcut `ctrl + r`:
Set vscode keyboard shortcut, run `ctrl + k` and `ctrl + s` and search for `rust.analyzer.run` and set `ctrl + r` as keyboard shortcut. 

