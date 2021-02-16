<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="./docs/images/logo.png" height="120" width="120" />
  </div>
  <h1 align="center">okku-server</h1>
  <h4 align="center">
    Back-End for Okku realtime chat solution. All the logic related
    to Okku's features are handled through this server. An instance
    of this server handles user authentication, media management, chat
    history as well as connections
  </h4>
</div>

## Index

- [Architecture](#architecture)
- [Front-End](#front-end)
- [Getting Started](#getting-started)
  - [Environment Variables](#environment-variables)
  - [Database](#database)
- [API Endpoints](#api-endpoints)
  - [About deprecated endpoints](#about-deprecated-endpoints)
- [License](#license)
- [Contributions](#contributions)

## Architecture

<div align="center" style="display: block; text-align: center;">
  <img src="./docs/images/diagram.png" width="1000" />
</div>

## Front-End

Theres two frontend solutions available for this project:

### [Okku CLI](https://github.com/EstebanBorai/okku-cli)

The Okku CLI offers a terminal based interface for sending and receiving
text messages.

## Getting Started

For development purposes you require Rust installed in your system and
Docker with `docker-compose` as well.

> It's recomended to install Rust using [rustup.rs](https://rustup.rs) solution.

### Environment Variables

Environment variables specified on `.env.sample` file are required by both,
the database and the server.

Create a `.env` file with a copy of the contents available on the `.env.sample`
file and fill the relevant values.

> Please make sure you have a `.env` file ready before going any further

### Database

The PostgreSQL database must be running in order for the server to run.
Please issue the following command:

```sh
docker volume create --name okku-database && docker-compose up --build
```
[Source](./bin/start-docker)

This command will create a new volume to store database files with the name
`okku-database`. And then will run `docker-compose up --build` using the
`docker-compose.yml` file available in the project's root directory.

### Server

In order to run the server you must issue:

```sh
cargo run
```

## API Endpoints

<table>
  <thead>
    <th>Name</th>
    <th>Description</th>
    <th>Method</th>
    <th>URI</th>
    <th>HTTP Headers</th>
    <th>HTTP Req. Body</th>
    <th>HTTP Res. Body</th>
  </thead>
  <tbody>
    <tr>
      <td>Login</td>
      <td>
        Authenticates an existing user and
        retrieves a JWT token
      </td>
      <td>GET</td>
      <td><code>/api/v1/auth/login</code></td>
      <td>
        <ul>
          <li>
            "Authorization: Basic {Base64(user_id:password)}"
          </li>
        </ul>
      </td>
      <td>N/A</td>
      <td>
        <code>
          {
            "token": ":JWT Token"
          }
        </code>
      </td>
    </tr>
    <tr>
      <td>Me</td>
      <td>
        Retrieve Profile and User details for
        the authenticated user
      </td>
      <td>GET</td>
      <td><code>/api/v1/auth/me</code></td>
      <td>
        <ul>
          <li>
            "Authorization: Bearer {Token}"
          </li>
        </ul>
      </td>
      <td>N/A</td>
      <td>
        <code>
          {
            "user": {
              "id": "52933f2f-2a2f-4942-8398-a8aee83569c6",
              "name": "foo"
            },
            "profile": {
              "id": "0bc1eefd-6dd1-48dc-be2d-73c94ba7f984",
              "first_name": null,
              "email": "foobar@okku.com",
              "avatar": null,
              "surname": null,
              "birthday": null,
              "contacts": null,
              "bio": null
            }
          }
        </code>
      </td>
    </tr>
    <tr>
      <td>Sign Up</td>
      <td>
        Registers a new user and retrieves
        a token
      </td>
      <td>POST</td>
      <td><code>/api/v1/auth/signup</code></td>
      <td>N/A</td>
      <td>
        <code>
          {
            "name": "foobar",
            "password": "root",
            "email": "foobar@okku.com"
          }
        </code>
      </td>
      <td>
        <code>
          {
            "token": ":Token",
            "user": {
              "id": "705c0c8f-9fc7-424d-a9c7-edc9df9146e0",
              "name": "foobar"
            }
          }
        </code>
      </td>
    </tr>
    <tr>
      <td>Find User Chats</td>
      <td>
        Retrive authenticated user chats
      </td>
      <td>GET</td>
      <td><code>/api/v1/chats</code></td>
      <td>
        <ul>
          <li>
            "Authorization: Bearer {Token}"
          </li>
        </ul>
      </td>
      <td>N/A</td>
      <td>
        <code>
          {
            "chats": [
              {
                "id": "10c941f5-f2cc-4f74-890b-34ad5c24fadd",
                "participants_ids": [
                  "56851552-eb2b-478b-8401-4abcd6754380",
                  "52933f2f-2a2f-4942-8398-a8aee83569c6"
                ]
              }
            ]
          }
        </code>
      </td>
    </tr>
    <tr>
      <td>Fetch Chat Messages</td>
      <td>
        Retrieve chat's message history
      </td>
      <td>GET</td>
      <td><code>/api/v1/chats/:chat_id/messages</code></td>
      <td>
        <ul>
          <li>
            "Authorization: Bearer {Token}"
          </li>
        </ul>
      </td>
      <td>N/A</td>
      <td>
        <code>
          {
            "messages": [
              {
                "id": "9fee900b-d92e-4e1e-ad35-b2593a7a53cb",
                "body": "Hello world!",
                "chat": {
                  "id": "10c941f5-f2cc-4f74-890b-34ad5c24fadd",
                  "participants_ids": [
                    "56851552-eb2b-478b-8401-4abcd6754380",
                    "52933f2f-2a2f-4942-8398-a8aee83569c6"
                  ]
                },
                "author": {
                  "id": "56851552-eb2b-478b-8401-4abcd6754380",
                  "name": "foobar"
                },
                "created_at": "2021-02-13T02:12:39.235418Z"
              }
            ]
          }
        </code>
      </td>
    </tr>
    <tr>
      <td>Create Chat</td>
      <td>
        Creates a new chat and specify its
        participants
      </td>
      <td>POST</td>
      <td><code>/api/v1/chats</code></td>
      <td>
        <ul>
          <li>
            "Authorization: Bearer {Token}"
          </li>
        </ul>
      </td>
      <td>
        <code>
          {
            "participants_ids": [
              "56851552-eb2b-478b-8401-4abcd6754380",
              "52933f2f-2a2f-4942-8398-a8aee83569c6"
            ]
          }
        </code>
      </td>
      <td>
        <code>
          {
            "id": "10c941f5-f2cc-4f74-890b-34ad5c24fadd",
            "messages": [],
            "participants_ids": [
              "56851552-eb2b-478b-8401-4abcd6754380",
              "52933f2f-2a2f-4942-8398-a8aee83569c6"
            ]
          }
        </code>
      </td>
    </tr>
  </tbody>
</table>

### About deprecated endpoints

If you look closely to domain directory, you will notice that theres
logic available for more features which is not exposed to the API.

These logic includes file management and profile details which used to
be part of a release of this server on a Web Application.

Those features are now deprecated in order to focus on main functionality
before going any further.

This project is part of my learning journey with Rust and implementing this
features helped me understand more how the language works and what it has
to offer.

If you are interested on working on some of these features, from the server
side or the client side, feel free to reach me via an issue I will be happy
to help!

## License

This project is licensed under the MIT license

## Contributions

All contributions are welcome! Feel free to open a pull request or an issue


