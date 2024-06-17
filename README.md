# ex3_ground_station

Ground station software to interface with Ex Alta 3

This (very) preliminary strawman uses Rocket to implement a simple WebServer
that can send messages to the OBC prototype in ex3_obc_fe2o3.

## Prerequisites

Ensure you have the following installed:

-   [Rust]
-   [PostgreSQL]
-   [Trunk]
-   [SeaORM CLI]

To build/run:

1. Install PostgreSQL: https://www.postgresql.org/download/
2. Create a Database: https://www.postgresql.org/docs/current/tutorial-createdb.html
3. Update the db_url in `main.rs` with username, password and db_name
4. Install SeaORM CLI: https://www.sea-ql.org/SeaORM/docs/index/
5. Use sea-orm-cli to run migrations: https://www.sea-ql.org/sea-orm-tutorial/ch01-02-migration-cli.html
6. run server: `cargo run`

This should launch the http-server on http://localhost:8000/index.html
The server currently expects the OBC to be listening on localhost:50000

# ex3_ground_station_dashboard

## Prerequisites

Ensure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Trunk](https://trunkrs.dev/#install)

## Running the Dashboard

To run the dashboard, follow these steps:

1. **Navigate to the Dashboard Directory:**
   navigate to the `ex3_ground_station_dashboard` directory.

    ```sh
    cd ex3_ground_station_dashboard
    ```

2. **Serve the Dashboard**
    ```sh
    trunk serve --open
    ```
    By default, Trunk will serve the application at http://127.0.0.1:8080. Open this URL in your web browser to view the dashboard.

# CLI to send a command to the OBC via TCP port

1. **Navigate to cli_command_obc**

```sh
    cd cli_command_obc
```

2. See the [README](./cli_command_obc/README.md) for usage and more info
