<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://via.placeholder.com/120" height="120" width="120" />
  </div>
  <h1 align="center">msend-server</h1>
  <h4 align="center">WebSocket server for msend a minimalistic messaging application made with ReactJS and Rust</h4>
</div>

## Motivation

`msend` is a chat application based on Web Sockets written in Rust
for the Back-End and ReactJS for the Front-End.

The main goal is to experiment with Web Sockets and the Rust programming
language, taking advantage of the Tokio runtime.

## Development

In order to run this application locally Rust must be installed in your system.
Its recommended to use [rustup](https://rustup.rs) to install Rust the first time.

You will need Docker as well, as the database is initialized through there, if you
don't want to install Docker you can make use of your local PosgtgreSQL installation.

### Requirements

- Docker
- Rust

### Setup

1. Clone the repository locally

```shell
https://github.com/EstebanBorai/msend-server.git
```

2. Execute the `bin/dotenv` script to create a `.env` file
or copy the contents of the `.env.sample` file into a new file
with the name `.env`

3. Run the Docker instance using the `bin/docker-start` script

```shell
bin/docker-start
```

4. When the server is ready, run migrations to make sure every
table on the database is available at the moment of connecting and
executing queries.

```shell
bin/sqlx-cli migrate run
```

5. Install dependencies and execute the server

```bash
RUST_LOG=info cargo run
```

If you get an output like the following:

```shell
 msend-server git:(main) RUST_LOG=info cargo run
   Compiling msend-server vx.x.x (/msend-server)
    Finished dev [unoptimized + debuginfo] target(s) in 35.83s
     Running `target/debug/msend-server`
[2020-12-24T23:34:18Z INFO  msend_server::database] Checking on database connection...
[2020-12-24T23:34:18Z INFO  sqlx::query] /* SQLx ping */; rows: 0, elapsed: 954.633µs
[2020-12-24T23:34:18Z INFO  sqlx::query] SELECT 1; rows: 1, elapsed: 2.440ms
[2020-12-24T23:34:18Z INFO  msend_server::database] Database PING executed successfully!
[2020-12-24T23:34:18Z INFO  msend_server] Server listening on: http://127.0.0.1:3000
```

You should be good to go, now move to the [API Reference](#api-reference) to get more
details on how to use MSend's API.

## Database Management

A database connection pool for the PostgreSQL database instance is
available and managed using the SQLX crate.

The `sqlx-cli` version included is `0.2.0`, which resides inside of the
`bin/` directory.

In order to create a migration you must execute:

```shell
bin/sqlx-cli migrate add <name>
```

Remember to keep an eye on new migrations, try to build the habit of
running migrations when you update your local version or before running
the project for development.

In order to run available migrations you must run:

```shell
# make sure the database is available on the same
# URL specified on the "DATABASE_URL" environment
# variable
bin/sqlx-cli migrate run
```

## API Reference

The server structure of this application is well known as _monolith_
thus, the same instance is capable to serve multiple domain services
such as authentication, chat, users, images and many more.

Some references on requests are available for [curl](https://github.com/EstebanBorai/msend-server/blob/main/docs/curl-requests.md) and [the browser](https://github.com/EstebanBorai/msend-server/blob/main/docs/browser-requests.md) as well.

### Auth

Description | URI | Method | HTTP Headers | Req. Body | Res. Body
--- | --- | --- | --- | --- | ---
Authenticate an existent user and retrieve a token | `auth/login` | GET | `Authorization: Basic <Basic Auth>` | N/A | `{"status_code": <status code>, "payload": { "token": <JWT Token> }}`
Create a new user and retrieve a token | `auth/signup` | POST | N/A | `{"name": "username", "password": "password"}` | `{"status_code": <status code>, "payload": { "token": <JWT Token> }}`

### Chat

Description | URI | Method | HTTP Headers | Req. Body | Res. Body
--- | --- | --- | --- | --- | ---
Connect to WebSocket to receive and send messages | `chat?token=<JWT Token>` | This endpoint makes use of the `WebSocket` (ws://) protocol | N/A | N/A | N/A

### Users

Description | URI | Method | HTTP Headers | Req. Body | Res. Body
--- | --- | --- | --- | --- | ---
Download an avatar | `api/v1/users/avatar/{user id}` | GET | `Authorization: Bearer <Token>` | N/A | `<File>`
Upload an avatar | `api/v1/users/avatar/{user id}` | POST | `Authorization: Bearer <Token>` | `FormData: image=<File>` | `<File>`
Replace an existent avatar | `api/v1/users/avatar/{user id}` | PUT | `Authorization: Bearer <Token>` | `FormData: image=<File>` | `<File>`

## References

These are some articles and tutorials that could help you getting
started with Rust and Warp.

* [Create an async CRUD web service in Rust with warp](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/)
* [Building a Real-time Chat App in Rust and React](https://outcrawl.com/rust-react-realtime-chat)
* [Let's make a simple authentication server in Rust with Warp](https://blog.joco.dev/posts/warp_auth_server_tutorial)
* [File upload and download in Rust](https://blog.logrocket.com/file-upload-and-download-in-rust/)
