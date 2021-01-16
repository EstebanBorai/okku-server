<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./docs/logo.png" height="120" width="120" />
  </div>
  <h1 align="center">hoth-server</h1>
  <h4 align="center">
    The Hoth project aims to develop a realtime chat application using WebSocket and asynchronous channels from Tokio. This repository contains the Back-End logic for the application
  </h4>
</div>

<div align="center">

  ![Build](https://github.com/EstebanBorai/hoth-server/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/hoth-server/workflows/clippy/fmt/badge.svg)

</div>

## Architecture

<div align="center" style="display: block; text-align: center;">
  <img src="./docs/diagram.png" width="1000" />
</div>

## Getting Started

To run this project locally you will need to have Rust installed
in your system, as well as Docker.

You will probably need two terminal windows/tabs to run this application, the first
tab will run `docker-compose` and the second will wake up the Rust/WARP server.

First of all, create a `.env` file, a file with predefined values is created if you run
`bin/dotenv` script. Otherwise you can create one from the `.env.sample` file.

To run the PostgreSQL database you must run `docker-compose up --build` from the
project's root directory.

> If you are using a UNIX based system you can also make use of the handy script `bin/docker-start`

When your Docker instance is ready and running you must initialize your server,
run `cargo run` to wake up the server.

If everything is fine (and you made use of the `dotenv` script), your Rust server instance
must be available from `http://127.0.0.1:3000` address in your system, as well as the PostgreSQL
instance from Docker at the `http://127.0.0.1:5432` address.
