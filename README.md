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

Once that's done, you can go to the root of the project and run the server with

```BASH
cargo run -p server
```

Once the server is running, in an other terminal, you can run either cli, gui or both with the same command ``cargo run -p <the_service_you_want>``.

## Available commands

```BASH
cargo build -p <service>    - Build the binary of the chosen service at the path target/debug/<service>
cargo run -p <service>      - Build and run the binary of the chosen service
cargo clippy -p <service>   - Lint the chosen service
cargo clean                 - Delete the 'target' folder.
```

# Resources

- **Rust** : [The french version of The Book](https://jimskapt.github.io/rust-book-fr/), [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)
- **Tokio** : [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- **Ratatui** : [Ratatui website](https://ratatui.rs/)

## AI Usage

Generative AI tools were used during development for:

- Debugging
- Understanding Rust concepts when documentation isn't clear enough

# Architecture
 <!-- Rémi -->
# Protocol Implementation
 <!-- Rémi -->
# Combat System

## Turns

When a fight begins, the server creates a fighters list with the first one. Each time a new player enter in the fight, he is added to the list. The turns order is determinate according to the list order.

The first on the list play at first, then it's the second, etc... When the last player finish his turn, it's enemy's turn. Then, the counter come back to 0 and we start over.

## Damages

The basic attack deals 15 damages to the enemy. It can be increase by weapons. You deal as much damage as your best weapon. The enemies don't have any defenses.
When it's enemy's turn, if there is just one player, he is the target (obviously) but if there are more than one, I retrieve the exact time since the 1 January 1970 and I extract the nanoseconds of this time. 

Next, I calculate the remainder when these nanoseconds are divided by the number of fighters so I have a result between 0 and number_of_fighters - 1. And I use it to chose a target in the fighters list.

The target takes damage equal to the enemy's damage value minus the armor value of its best piece of armor.
If the enemy has a damage attribut of 35 and the target has an helmet with an armor value of 15 and a shield with an armor value of 20, the target will takes 15 damages (35 - 20).

## Commands

The player has three options during their turn:
- **Attack :** Attack the enemy with 15 damages or the damages value of their best weapon.
- **Bag :** Take and consume a potion in their bag.
- **Flee :** He flees the fight at the cost of 10 gold pieces and his dignity.

# Quest System

There are four types of quests:
- Retrieve and deliver an item to an NPC
- Collect specific items
- Talk to an NPC
- Answer a riddle.

The player structure has two attributes for quest management: 
- **quests_in_progress**: a HashMap with the quest ID as key and its progress step as value
- **finished_quest**: a HashSet containing the IDs of completed quests.

When a player accepts a quest from an NPC, if the quest ID is neither in quests_in_progress nor in finished_quests, it is added to quests_in_progress with a step value of 0.

After each player request, at the end of the handle_request function, the progress of active quests is evaluated.

If the request satisfies a quest goal, the quest is updated in quests_in_progress. The step is incremented by one, and an update event is sent to the client.

If the current step equals the total number of goals, the quest is marked as finished, and a completion event is sent to the client.

The reward is then added to the player's inventory, and the quest ID is moved to finished_quests.

# World Design

# Server Logging
 <!-- Rémi -->
# Group Contributions
 <!-- Matisse -->
# Building and Running
 <!-- Matisse -->
# Testing
 <!-- Matisse -->
