# bankingv2

`bankingv2` is an event-sourced banking application built with Rust. It utilizes SQLite as its event store and Kafka as its event bus.

## Core Technologies

* **Rust**: The primary programming language.
* **SQLite**: Used as the event store to persist all domain events.
* **Kafka**: Used as the event bus for publishing and subscribing to domain events.
* **Docker**: For running Kafka and Zookeeper.
* **Just**: A command runner for managing project tasks like formatting and linting.

## Prerequisites

Before you begin, ensure you have the following installed:

* [Rust](https://www.rust-lang.org/tools/install)
* [Docker and Docker Compose](https://docs.docker.com/get-docker/)
* [Just](https://github.com/casey/just#installation)

## Getting Started

### 1. Set up Kafka

The project uses Docker Compose to manage Kafka and Zookeeper services.

1. Navigate to the project root directory.
2. Start the Kafka services:

    ```bash
    docker-compose up -d
    ```

    This will start:
    * Zookeeper on port `2181`
    * Kafka on port `9092`
    * Kafdrop (a Kafka UI) on port `9000`, accessible at `http://localhost:9000`.
    * A `kafka-setup` service will automatically create an `events` topic in Kafka.

### 2. Run the Application

Once Kafka is running, you can build and run the Rust application:

1. **Build the project:**

    ```bash
    cargo build
    ```

2. **Run the application:**

    ```bash
    cargo run
    ```

    The application will connect to the Kafka instance defined in `docker-compose.yml` and use a local SQLite database for its event store (likely created in the project's `target` directory or a specified path if configured).

## Development

This project uses `just` as a command runner for common development tasks.

* **List available commands:**

    ```bash
    just
    ```

* **Check for linting errors:**

    ```bash
    just check
    ```

* **Format code and fix linting errors:**

    ```bash
    just fix
    ```

## Project Structure

* `src/`: Contains the Rust source code.
  * `main.rs`: The main application entry point.
  * `event_store_sqlite.rs`: Implementation for the SQLite event store.
  * `event_bus_kafka.rs`: Implementation for the Kafka event bus.
  * `account.rs`: Domain logic for accounts.
  * `traits.rs`: Common traits.
* `Cargo.toml`: Rust project manifest, defining dependencies and metadata.
* `Cargo.lock`: Records exact versions of dependencies.
* `justfile`: Defines `just` commands for task automation.
* `docker-compose.yml`: Defines and configures Docker services (Kafka, Zookeeper, Kafdrop).
* `.gitignore`: Specifies intentionally untracked files that Git should ignore.

## Event Sourcing Overview

This application implements the event sourcing pattern:

* **Events**: All changes to the application state are captured as a sequence of immutable events.
* **Event Store**: Events are durably stored in SQLite (`event_store_sqlite.rs`).
* **Event Bus**: After being persisted, events are published to a Kafka topic (`events`) via `event_bus_kafka.rs`. Other services or components can then subscribe to these events to react accordingly.

This architecture allows for robust auditing, easy debugging, and the ability to replay events to reconstruct state or build new projections.
