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

The implementation of the Back-End servier is based on
[Tin Rabzelj's article "Building a Real-time Chat App in Rust and React"](https://outcrawl.com/rust-react-realtime-chat).

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
