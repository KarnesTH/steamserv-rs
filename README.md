# Steamserv

A CLI (Command Line Interface) for Steam game servers. A easy way to install and manage game servers locally on a Linux machine.

## Features

- Install game servers
- Update game servers
- Start game servers
- Stop game servers
- Restart game servers
- Uninstall game servers
- List installed game servers
- List available game servers
- Generate .service files for systemd

## Installation

1. Clone the repository

```bash
git clone https://github.com/KarnesTH/steamserv-rs.git
```

2. Change directory to the repository

```bash
cd steamserv-rs
```

3. Build the project

```bash
cargo build --release
```

4. Install the binary

```bash
sudo cp target/release/steamserv /usr/local/bin
```

or you can use the `cargo install` command

```bash
cargo install --path .
```

## Usage

### Install a game server
```bash
// Install a game server with specific appid, server name and steam user name
steamserv-rs install --appid <steam app id> --server-name <folder server name> --username <steam user name>
```
```bash
// User interactive mode to install a game server
steamserv-rs install
```

### List game servers
```bash
// List all available game servers
steamserv-rs list
```
```bash
// List all installed game servers
steamserv-rs list --installed
```
```bash
// List all available game servers that match the filter
steamserv-rs list --filter <server name>
```

### Update a game server
```bash
// Update a game server with specific server name
steamserv-rs update --server-name <server name>
```
```bash
// User interactive mode to update a game server
steamserv-rs update
```

### Uninstall a game server
```bash
// Uninstall a game server with specific server name
steamserv-rs uninstall --server-name <server name>
```
```bash
// User interactive mode to uninstall a game server
steamserv-rs uninstall
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
