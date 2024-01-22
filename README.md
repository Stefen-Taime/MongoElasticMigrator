# MongoDB to Elasticsearch Migrator

## Introduction
This tool migrates data from MongoDB collections to Elasticsearch indices. It's built using Rust and supports configurable migrations.

## Features
- Migrate multiple MongoDB collections.
- Configurable MongoDB and Elasticsearch connections.
- Filter data during migration.

## Configuration

To configure the migrator, edit the `config.toml` file located in the root of the project. This file contains settings for MongoDB and Elasticsearch connections, as well as additional options.

### Structure of `config.toml`

```toml
[mongodb]
uri = "mongodb://username:password@host:port"
database_name = "your_database_name"
collection_name = "your_collection_name"
# Optional filter (in JSON format) to select specific documents
filter = "{\"key\": \"value\"}"

[elasticsearch]
uri = "http://elasticsearch_host:port"
index_name = "your_index_name"


## Usage

Before using the migrator, ensure Rust, OpenSSL, and pkg-config are installed on your system.

### Prerequisites

1. **Install Rust**: 
   Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust.

### Rust Installation on Linux or macOS

To install Rust, including both the Rust compiler (`rustc`) and Cargo (the Rust package manager), follow these steps:

1. Open a terminal.
1a. Execute the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

   - rustc --version
   - cargo --version
  

2. **Install OpenSSL**: 
   - Ubuntu: `sudo apt-get install libssl-dev`
   - Fedora: `sudo dnf install openssl-devel`

3. **Install pkg-config**:
   - Ubuntu: `sudo apt-get install pkg-config`
   - Fedora: `sudo dnf install pkg-config`

### Running the Migrator

After installing the prerequisites, configure the tool using `config.toml` and run it:

```bash
cargo run


