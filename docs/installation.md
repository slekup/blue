# Installation

## On Windows

```powershell
iwr https://raw.githubusercontent.com/slekup/blue/main/install.ps1 -useb | iex
```

## On Linux and macOS

```bash
curl -fsSL https://raw.githubusercontent.com/slekup/blue/main/install.sh | sh -
```

or with `wget`

```bash
wget -qO- https://raw.githubusercontent.com/slekup/blue/main/install.sh | sh -
```

## Crates.io

If you would like to easy build the source code yourself, installing Blue from [crates.io](https://crates.io/crates/blue-cli) will not include the pre-built binaries.

```bash
cargo install blue-cli
```