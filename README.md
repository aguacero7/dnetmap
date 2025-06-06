# dnetmap

`dnetmap` is a lightweight, fast, and readable CLI tool to **inspect Docker networks** on your local machine.

It extracts key information such as:
- Network name and ID
- IP range (subnet)
- Associated Linux bridge interface
- Whether the network is linked to a Docker Compose project (and if so, which one)

Perfect for troubleshooting containers, auditing Compose environments, or visualizing your local network topology.

## Installation

### Prerequisites

- [Rust](https://rust-lang.org) (edition 2021 or later)
- Docker CLI installed and accessible (`docker network inspect`, etc.)

### Build manually

```bash
git clone https://github.com/aguacero7/dnetmap.git
cd dnetmap
cargo build --release
```
The binary will be available at:
```bash
./target/release/dnetmap
```
You will usually MUST execute it as root to be able to communicate with docker

## Usage
Basic usage
```bash 
dnetmap all
```

Example output
```bash
Network name     : app_default
ID               : a1b2c3d4e5f6
IP range         : 172.20.0.0/16
Linux interface  : br-a1b2c3d4e5f6
Linked to Compose: yes, project `app`
------------------------------------------------------------
```

## Error handling
- If docker is **not running** or **not installed**, you'll see a clear error message
