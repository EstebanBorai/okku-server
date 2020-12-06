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

1. Clone the repository locally

```bash
https://github.com/EstebanBorai/msend-server.git
```

2. Install dependencies and execute the server

```bash
cd msend-server

RUST_LOG=info cargo run
```

A [warp](https://github.com/seanmonstar/warp) server will listen on `ws://127.0.0.1:8080/`.


## Getting Started

You must complete all steps on [Development](#development) section in order
to follow the steps on this section.

## API Reference

Description | URI | Method | HTTP Headers | Req. Body | Res. Body
--- | --- | --- | --- | --- | ---
Authenticate an existent user and retrieve a token | `auth/login` | GET | `Authorization: Basic <Basic Auth>` | N/A | `{"status_code": <status code>, "payload": { "token": <JWT Token> }}`
Create a new user and retrieve a token | `auth/signup` | POST | N/A | `{"name": "username", "password": "password"}` | `{"status_code": <status code>, "payload": { "token": <JWT Token> }}`
Download an avatar | `api/v1/users/avatar/{user id}` | GET | `Authorization: Bearer <Token>` | N/A | `<File>`
Upload an avatar | `api/v1/users/avatar/{user id}` | POST | `Authorization: Bearer <Token>` | `FormData: image=<File>` | `<File>`
Replace an existent avatar | `api/v1/users/avatar/{user id}` | PUT | `Authorization: Bearer <Token>` | `FormData: image=<File>` | `<File>`

### Sending a message

With the server running, on `ws://127.0.0.1:8080/`, a WebSocket connection
must be established from the client side.

Open your favorite browser, then open the developer tools (usually by pressing F12),
and write the following to the console.

```javascript
// create a WebSocket connection using the WebSocket object
const msend = new WebSocket('ws://127.0.0.1:8080');

// send a message
msend.send(JSON.stringify({
  type: 'post',
  payload: {
    body: 'Hello from msend!'
  }
}));
```

## References

These are some articles and tutorials that could help you getting
started with Rust and Warp.

* [Create an async CRUD web service in Rust with warp](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/)
* [Building a Real-time Chat App in Rust and React](https://outcrawl.com/rust-react-realtime-chat)
* [Let's make a simple authentication server in Rust with Warp](https://blog.joco.dev/posts/warp_auth_server_tutorial)
* [File upload and download in Rust](https://blog.logrocket.com/file-upload-and-download-in-rust/)
