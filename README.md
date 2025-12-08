# Overview

This is a personalized rust study note.

## Init project with cargo

`cargo new <project-name>`


## Vscode Setup Rust Analyzer:
Install the rust analyzer plugin in vscode. and link the analyzer executable using 

`"rust-analyzer.server.path": "/path-to-the-analyzer",`
entry in vscode setting.json. (Find the analyzer executable using `which rust-analyzer` command

For Termux in android, install rust analyzer using `pkg` and link that.`"rust-analyzer.server.path": "/data/data/com.termux/files/usr/bin/rust-analyzer",`

