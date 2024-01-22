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

```


## Usage

Before using the migrator, ensure Rust, OpenSSL, and pkg-config are installed on your system.

### Prerequisites

1. **Install Rust**: 
   Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust.

### Rust Installation on Linux or macOS

To install Rust, including both the Rust compiler (`rustc`) and Cargo (the Rust package manager), follow these steps:

1. Open a terminal.
2. Execute the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  

2. **Install OpenSSL**: 
   - Ubuntu: `sudo apt-get install libssl-dev`
   - Fedora: `sudo dnf install openssl-devel`

3. **Install pkg-config**:
   - Ubuntu: `sudo apt-get install pkg-config`
   - Fedora: `sudo dnf install pkg-config`

### Running the Migrator


After running `cargo run`, if the migration is successful, you should see output similar to the following:

This output indicates that:
- Each "Document successfully indexed" message corresponds to a single document that has been successfully transferred to Elasticsearch.
- "Total documents indexed to Elasticsearch: xxx" shows the total number of documents that were migrated.
- "Data migration completed in X.XXXs" indicates the total time taken for the migration process.

## Using Docker for MongoDB and Elasticsearch

If you don't have MongoDB and Elasticsearch set up, you can use the provided Docker configurations to start these services:

1. Navigate to the `docker` directory:
   ```bash
   cd docker

## License

This project is licensed under the MIT License - see the LICENSE file for details.

MIT License

Copyright (c) [2024] [Stefen-Taime]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


```bash
cargo run


