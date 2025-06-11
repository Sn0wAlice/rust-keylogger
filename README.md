# Rust Keylogger 

> ⚠️ This project is intended **strictly for educational and detection testing purposes**. Do not use in any real-world environment without explicit authorization.

## Overview

This is a simple keylogger written in Rust using the [`rdev`](https://crates.io/crates/rdev) crate. It captures key presses and logs them to a file.

## Features

- Captures global key presses
- Logs keys to `keylog.txt`
- Works on Linux, macOS, and Windows (with proper permissions)

## Requirements

- Rust (https://rustup.rs)
- Permissions to access keyboard events:
    - macOS: Enable accessibility permissions
    - Linux: Requires X11
    - Windows: May require administrator privileges

## Installation

```bash
git clone https://github.com/Sn0wAlice/rust-keylogger
cd rust-keylogger
cargo build --release
```

## Usage

```bash
cargo run --release
```

This will start listening for key presses and save them to `keylog.txt`.

## Detection Tips for SIEM

Use this tool to generate telemetry and validate detection strategies:

- Monitor access to `rdev`, `X11`, or `SetWindowsHookEx` APIs
- Watch for processes writing to files like `keylog.txt` outside standard paths
- Look for CLI apps without GUI accessing keyboard input
- Detect Rust-based binaries hooking into keyboard events

## Disclaimer

This software is provided for educational and research purposes only. You are solely responsible for using it in accordance with applicable laws and organizational policies.
