# EX3 Ground Station

## Overview

The EX3 Ground Station software interfaces with the Ex Alta 3 satellite, offering command and control capabilities through a web-based dashboard and a command-line interface. The project is organized as a Rust workspace with multiple components including a backend server, a web dashboard, migration tools, and a CLI for direct OBC communication.

## Workspace Structure

-   **cli_command_obc**: Command-line interface for direct communication with the satellite's on-board computer (OBC).
-   **dashboard**: Web-based UI dashboard for monitoring and managing satellite operations.
-   **migration**: Contains database migration scripts to manage the evolution of database schema.
-   **server**: Backend server that provides APIs for the dashboard and handles all server-side logic including database interactions.

## Getting Started

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)
-   [PostgreSQL](https://www.postgresql.org/download/)
-   [Trunk](https://trunkrs.dev/#install)
-   [SeaORM CLI](https://www.sea-ql.org/SeaORM/docs/installation/cli)

### Installation and Setup

1. **Clone the repository:**

-   `git clone https://github.com/AlbertaSat/ex3_ground_station.git`
-   `cd ex3_ground_station`

2. **Configure the environment variables:**

-   Create a `.env` file in the root directory.
-   Update the `DATABASE_URL` in the `.env` file to point to your PostgreSQL database. For the official database URL, please contact the lead developer.

3. **Run database migrations (if necessary):**

-   If you are setting up a new database instance or need to update an existing database to reflect recent changes in the schema:
    ```
    cd migration
    sea-orm-cli migrate up
    ```

### Run Server

1. `cd server`
2. `cargo run`

### Run Dashboard

1.  `cd dashboard`
2.  `trunk serve`

### Run CLI

1. `cd cli_command_obc`
2. `cargo run`
