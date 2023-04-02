# Chat

A chat server built on top of TCP

---

## Installation

Make sure that you [install Rust](https://www.rust-lang.org/tools/install) first.

Then use [Cargo](https://doc.rust-lang.org/cargo/) to install the server and client binaries:

### Install the client

```bash
cargo install --git https://github.com/wcygan/chat client
```

This will install `chatcli`, the client binary

### Install the server (Optional)

If you are just interested in using the client, you can skip this step. 

Installing the server will allow you to run your own chat server.

```bash
cargo install --git https://github.com/wcygan/chat server
```

This will install `chatsrv`, the server binary

## Usage

### Client

A client allows you to connect to a chat server and send messages to other clients.

```
chatcli --help
A client used to connect to a chat server

Usage: chatcli [OPTIONS]

Options:
  -a, --address <ADDRESS>  The address of the server to connect to [default: 127.0.0.1:8080]
  -h, --help               Print help
```

### Server

A server allows you to spawn a chat server on a given port. Clients can connect to this server and send messages to each other.

```
chatsrv -h
Spawn a chat server on the given port

Usage: chatsrv [OPTIONS]

Options:
  -p, --port <PORT>  The port to use for the server [default: 8080]
  -h, --help         Print help
```