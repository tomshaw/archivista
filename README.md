# Archivista 🚀

![Build](https://github.com/tomshaw/archivista/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/d/archivista.svg)](https://crates.io/crates/archivista)
[![Crates.io](https://img.shields.io/crates/v/archivista.svg)](https://crates.io/crates/archivista)

A command-line application for quickly backing up your databases. It supports exporting multiple databases in a single operation, runs asynchronously and is extremely fast. Outputs export information as each operation completes. Tested on moderate to large size databases without any problems.

## Features

- Export all your database in a single operation.
- Supports `mysql`, `postgres` and `sqlserver`.
- Runs asynchronously, and is extremely fast.
- Exclude selected databases from the backup.
- Prints exports sorted by duration in a colorized table.

## Usage

1. Set the following environment variables:

    - `DB_CONNECTION`: The type of server. Use `mysql`, `postgres` or `sqlserver`.
    - `DB_HOST`: The hostname of your server.
    - `DB_PORT`: The port number of your server.
    - `DB_USERNAME`: The username to use when connecting to your server.
    - `DB_PASSWORD`: The password to use when connecting to your server.
    - `DB_EXPORTS`: A comma-separated list of databases to backup. Use `*` to backup all databases.
    - `DB_FORGETS`: A comma-separated list of databases to exclude from the backup.

2. Run the application:

    ```bash
    cargo run
    ```

## Dependencies

This utility depends on the following Rust crates:

- `serde`: To deserialize the database configuration. Version: 1.0 with features: ["derive"]
- `tokio`: A runtime for writing reliable, asynchronous, and slim applications. Version: 1 with features: ["full"]
- `tokio-util`: Utilities for working with Tokio. Version: 0.6 with features: ["compat"]
- `tokio-postgres`: A native, asynchronous PostgreSQL driver. Version: 0.7
- `tiberius`: A native, asynchronous TDS implementation for Microsoft SQL Server. Version: 0.5
- `dotenv`: To load the database configuration from environment variables. Version: 0.15.0
- `mysql`: To connect to the MySQL server and retrieve the list of databases. Version: 24.0.0
- `colored`: To colorize the output to the terminal. Version: 2.0.4
- `cli-table`: To print the list of databases in a neat table. Version: 0.4.7
- `zip`: Reading and writing ZIP archives. Version: 0.5
- `futures`: Zero-cost futures in Rust. Version: 0.3


## Contributing

Contributions are welcome! Please submit a pull request or create an issue on GitHub.

## License 

The MIT License (MIT). See [License File](LICENSE) for more information.
