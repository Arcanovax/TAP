*This project has been created as part of the 42 curriculum by mthetcha, relaforge, bfitte.*

# Description

**The Answer Protocol** is a project that aims to learn us create our own small, persistent-feeling world where multiple players can explore rooms, chat, and cooperate in real time.

Our server will speak a simple, line-based TCP protocol, and our two clients — one command line and one graphical — will bring that world to life.

### Goal

The goal of this project is to build a Multi-User Dungeon (MUD) — a shared-world retro text adventure.

The primary technical objective is to design a TCP server capable of handling multiple concurrent connections while executing asynchronous code to manage real-time events.

The project is divided into three main components: 
- **Server:** Manages the persistent game world, static data, and asynchronous interactions between players.
- **CLI Client:** Allows users to interact with the game via terminal commands, running asynchronously to process and display events sent by the server.
- **GUI Client:** Provides the same real-time functionality and asynchronous event handling as the CLI, but within a graphical interface.  

### Overview

Client-server communication takes place via the RFC 42TAP protocol specified in the assignment.

It has been slightly modified to suit our needs, but the basic protocol has been followed.

The world managed by the server is created from data retrieved from the YAML files in the project's root directory.

All parts of the project (server, CLI, GUI) are in Rust so the entire project is managed by Cargo, Rust's package manager.

# Instructions

You must have version 1.96 of Rust to run the program. If you don't, just follow the next instructions:

Download and install the last stable version of rustup

```BASH
sudo apt install rustup
rustup default stable
```

Then, check the version

```BASH
rustc --version
```

You must have

```BASH
rustc 1.96.0 (ac68faa20 2026-05-25)
```

