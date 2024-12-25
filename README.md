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

```bash
steamserv-rs install --appid <steam app id> --path <path to install the server>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
```
