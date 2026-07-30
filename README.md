*This project has been created as part of the 42 curriculum by mthetcha, relaforg, bfitte.*

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

You must have version 1.96+ of Rust to run the program. If you don't, just follow the next instructions:

Download and install the last stable version of rustup

```BASH
sudo apt install rustup
```

or

```BASH
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


```BASH
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

or higher

Once that's done, you can go to the root of the project and run the server with

```BASH
cargo run -p server <config_entry_point>
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

- **Rust** : [The french version of The Book](https://jimskapt.github.io/rust-book-fr/), [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html), [Rustlings](https://rustlings.rust-lang.org/)
- **Tokio** : [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- **Ratatui** : [Ratatui website](https://ratatui.rs/)

## AI Usage

Generative AI tools were used during development for:

- Debugging
- Understanding Rust concepts when documentation isn't clear enough

# Architecture

## Overview

The server is a Tokio TCP server built around three ideas: **one task per connection**, **one shared world behind a mutex**, and **one central dispatcher** that turns a text line into a handler call.

**Connection lifecycle.** The accept loop spawns one task per client; that task then loops on a `select!` with two arms — the socket and the player's event queue.

```
   listener.accept()  ──►  tokio::spawn  ──►  one task per connected client
                                                        │
      ┌─────────────────────────────────────────────────┴───────────────────┐
      │  loop { select! {                                                   │
      │      read_line(socket)  ──►  a command arrived   → command path     │
      │      rx.recv()          ──►  an event arrived    → sent to socket   │
      │  } }                                                                │
      └─────────────────────────────────────────────────────────────────────┘
```

**Command path** — everything below happens inside that same task, synchronously:

```bash
   "MOVE north\n"
        │
        │  parse_command            (lib.rs)
        ▼
   Message::Command { name: "MOVE", args: ["north"] }
        │
        │  handle_request           (handlers/handle_request.rs)
        │    · in-fight / in-dungeon guards
        │    · Command::parse → exhaustive match
        ▼
   handlers/movement.rs
        │
        │  lock()  ──────────────►  ┌────────────────────────────────┐
        │  read the room, move      │     Arc<Mutex<ServerInfo>>     │
        │  the player, queue the    │  world · connections · groups  │
        │  ROOM_LEAVE/ROOM_JOIN     │      fights · dungeons         │
        │  events, drop the guard   └────────────────────────────────┘
        ▼
   HandlerOutcome { message, event }
        │
        │  advance_quests(event)    (state/quest.rs, runs for every command)
        ▼
   Message::Response ──► to_str() ──► "OK room=loc.beach\n" ──► socket
```

**Event path** — how another player learns about it. The handler never writes to a foreign socket: it pushes into that player's channel, and *their* task does the writing.

```bash
   task of player A (running the handler)      task of player B (idle in select!)
   ──────────────────────────────────────      ─────────────────────────────────
   get_room_receivers(A)
        │  &[Connection B, …]
        ▼
   con.tx.send(Event::ROOM_JOIN) ──► mpsc ──►  rx.recv()
                                                    │
                                                    ▼
                                               to_str() ──► socket B
                                        "EVT ROOM PRESENCE ENTER alice\n"
```

## Dispatcher / router, not inline handling

Command handling is **centralised in a router** (`server/src/handlers/handle_request.rs`) rather than inlined in the read loop:

1. `parse_command` (`lib.rs`) splits the line on whitespace: first token = command name, rest = `args`.
2. `Command::parse` resolves the name against a `Command` enum (`strum::EnumString` with `ascii_case_insensitive`, so `LOOK`, `look` and `Look` are the same command). An unknown token yields `ERR 903 INVALID_COMMAND` — the parser never panics on garbage input.
3. `handle_request` matches on that enum and delegates to one handler module per command (`handlers/look.rs`, `handlers/movement.rs`, …), each with its own sibling `tests.rs`.

Why a router:

- **The `match` on a `Command` enum is exhaustive.** Adding a variant makes the compiler point at the dispatcher until the command is wired — a whole class of "command silently ignored" bugs disappears.
- **Cross-cutting rules live in one place.** Two of them are implemented as pre-dispatch guards: a player `InFight` may not `TAKE/DROP/QUEST/BUY/SELL/TALK/MOVE`, and a player inside a dungeon may not `GROUP LEAVE`. Both return `ERR 404 FORBIDDEN_ACTION` before any handler runs, instead of being re-checked in seven handlers.
- **The quest engine is a post-dispatch hook.** Handlers return a `HandlerOutcome { message, event: Option<GameEvent> }`; after dispatch, `advance_quests` replays that event against every quest in progress. Quest logic is therefore written once in `state/quest.rs`, not scattered across every handler that could satisfy a goal.
- **Handlers stay synchronous and pure-ish.** A handler takes `&SharedServer` and returns a `Message`; it does no I/O. That is what makes them unit-testable without a socket (see `test_utils.rs`).

The trade-off we accepted: the dispatcher is a large `match` and every handler signature goes through it, so a change of signature touches one long file.

## Concurrency model

**One Tokio task per client.** After `listener.accept()`, the connection is moved into `tokio::spawn`. Each task owns its socket, split into a `BufReader` read half and an owned write half, so nothing is shared between clients at the socket level.

**Each connection task runs its own `select!`** over two futures: reading the next command line, and receiving on an `mpsc::UnboundedReceiver<Message>`. That is what makes the world feel live: a client sitting idle at a prompt still receives `EVT …` lines pushed by other players' actions, because the task is never blocked on `read_line` alone.

**Events are routed through per-connection channels, never by writing to someone else's socket.** `ServerInfo` stores a `Connection { addr, tx, player }` per player; broadcasting is `con.tx.send(Message::Event(…))` for every receiver returned by `get_room_receivers` / `get_group_receivers` / `get_global_receivers` (`state/broadcast.rs`). The sender's task therefore never touches another task's write half, and a slow client can never block the player who triggered the event. The channel is unbounded, so `send` cannot fail on backpressure and events are never dropped — at the cost of unbounded memory if a client stopped reading.

**Shared state is a single `Arc<Mutex<ServerInfo>>`** (`state.rs`), holding the world, connections, groups, fights and dungeons. Two deliberate choices here:

- **`std::sync::Mutex`, not `tokio::sync::Mutex`.** No handler is `async` and no `.await` ever happens while the guard is alive, so the lock is only ever held for a few microseconds of pure computation. An async mutex would add machinery for a contention that does not exist, and the compiler enforces our rule for us: holding a non-`Send` guard across an `.await` would not compile in a spawned task.
- **One global lock rather than fine-grained locks per room/player.** A MUD command routinely touches several aggregates at once (move = read a room, mutate a player, notify two rooms), so per-entity locks would mean lock ordering and deadlock risk for no measurable gain at this scale. Critical sections are kept short by locking inside a block and dropping the guard before writing to the socket.

**Two more tasks run alongside the clients:**

- A **background ticker** (`interval(600s)`) that resets the world — respawning merchant stock and defeated enemies, keeping items dropped by players — persists it to redb, and pushes `EVT SERVER RESET` to everyone.
- The **accept loop itself is a `select!`** over `listener.accept()` and `SIGINT`/`SIGTERM`. On either signal, `server_shutdown` saves the world and flushes every connected player to the database before returning, so `Ctrl-C` is a clean shutdown rather than a data loss.

## State, configuration and persistence

The world has two sources of truth, on purpose:

- **The YAML configuration is structural authority.** `config::load` walks an entry-point file (`server/config.yaml`) and builds the `World`: rooms, exits, items, NPCs, quests. Rooms and dialogues only ever come from there.
- **redb holds what players changed.** Two tables (`players`, `world`), values encoded with `bincode` through a custom `redb` type wrapper (`persistence/bincode.rs`). A player is loaded on `CONNECT` and saved on disconnect; the world is saved on every tick and at shutdown.

At boot, the config-built world is snapshotted into `base_world` **before** the saved items are merged in, and only items owned by `Owner::Player` are restored. That ordering is the invariant that keeps the periodic reset idempotent: reset restores `base_world`'s items and keeps the players', so nothing is duplicated tick after tick.

Dungeons are the one part of the world that is not in the config: they are generated at runtime (`dungeon/generation.rs`), stored per-`Uuid` in `ServerInfo.dungeons`, and their room/NPC ids embed the dungeon id. `resolve_room` / `resolve_npc` parse that id and transparently look up either the shared world or the caller's dungeon instance — so handlers never need to know whether the player is in the persistent world or in an instance.

# Protocol Implementation

The server implements the line-based text protocol of **RFC 42TAP**: UTF-8, one message per line terminated by `\n`, three frame kinds — `OK …`, `ERR <code> <NAME>`, `EVT <category> <type> <data>`. The greeting sent on connect is `OK hello proto=1`.

The sections below document where we **deviate from the RFC** and why.

## 1. Extension commands

The RFC defines the core verbs (`CONNECT`, `QUIT`, `WHO`, `CHAT`, `GROUP`, `MOVE`, `LOOK`, `TAKE`, `DROP`, `INVENTORY`, `STATUS`, `TALK`, `ATTACK`, `QUEST`, `QUESTS`). We added commands for features the RFC does not cover: `FLEE`, `CONSUME`, `ANSWER`, `NPC`/`NPCS`, `ITEM`/`ITEMS`, `ROOM`/`ROOMS`, `QUEST_INFO`, `BUY`/`SELL`/`GOLD`, `DUNGEON`, `SLOT_MACHINE`, `DICES`, `HELP`.

**Justification.** The RFC constrains the *framing*, not the vocabulary. Every extension is encoded with the same `OK`/`ERR` framing, so a strictly-RFC client is unaffected: it simply never sends them. Rejecting them would have meant inventing a second, non-RFC channel for the economy, the dungeon and the mini-games — a worse deviation. `HELP` exists so the vocabulary is discoverable from a raw `nc` session, since the enum carries its own descriptions.

## 2. Extension events

Beyond the RFC's presence, chat, group and stats events, we emit: `EVT ROOM TAKE|DROP`, `EVT FIGHT ENTER|ATTACK|ENEMY|HEALING|LEAVE`, `EVT QUEST UPDATE|FINISH`, `EVT DUNGEON CREATE`, `EVT SERVER RESET`.

**Justification.** They keep the `EVT <category> <type> <data>` shape, so the parsing rule stays uniform. Two sub-choices:

- **Positional data for simple events** (`EVT ROOM TAKE alice sword`), because the RFC states the last field is free-form (`1*VCHAR`) and may contain spaces — which is exactly what item and player names need.
- **A JSON blob as data for structured events** (`EVT QUEST UPDATE {"quest":…,"goal":…,"previous_goal":…}`). A quest goal has several fields, and quest names contain spaces; positional encoding would have been ambiguous. JSON is already part of the RFC's payload vocabulary, so this borrows an existing mechanism rather than inventing one.
- **`EVT SERVER RESET`** has no RFC counterpart but is required by our 10-minute world reset: without it a client would keep displaying a room state that no longer exists.

## 3. Error codes

The RFC's code space is extended. Codes `201`, `301`, `401`–`403` keep their RFC meaning; we added `302 NO_DIALOG`, `405`–`411` (`NPC_NOT_HOSTILE`, `NO_QUEST_AVAILABLE`, `ALREADY_INVITED`, `NOT_ENOUGH_GOLD`, `GAME_LOSE`, `DUNGEON_ALREADY_IN_PROGRESS`, `NO_DUNGEON_IN_PROGRESS`) and a `9xx` transport/protocol class (`900 CONNECTION_FAILED`, `901 SEND_FAILED`, `902 INVALID_ARGS`, `903 INVALID_COMMAND`, `904 ALREADY_CONNECTED`, `905 DISCONNECTION_FAIL`).

Two consequences worth stating explicitly:

- **Several distinct conditions share code `404`** (`ITEM_NOT_FOUND`, `NPC_NOT_FOUND`, `ROOM_NOT_FOUND`, `FORBIDDEN_ACTION`, `NOT_YOUR_TURN`, `UNUSABLE_ITEM`, …). The numeric code stays the RFC's "not found / not allowed" class, and the **symbolic name carries the precision**: `ERR 404 NOT_YOUR_TURN` is unambiguous for a human and for a client that matches on the name. We preferred that to minting a dozen new numbers that no other implementation would understand.
- **The `9xx` class separates "your request is malformed" from "your request is invalid in the game world".** A client can decide to log the former and display the latter without a lookup table.


## 4. `ERR` frames carry no payload — with one exception

The RFC has no data field on `ERR`, so `Message::to_str` drops the payload on any error. **`409 GAME_LOSE` is the single exception**: it is rendered as `ERR 409 GAME_LOSE <text>` when a text payload is present.

**Justification.** Death is the one error that is also a game outcome the player must be told about (what killed them, what they lost); returning `OK` for a death would be worse, and a follow-up `EVT` would race with the response. It is a deliberate, documented, single-code deviation.

## 5. Payload shapes

Success payloads follow the RFC's hybrid model, chosen per command:

| Form | Commands |
|---|---|
| bare `OK` | `CHAT`, `GROUP INVITE`, `GROUP LEAVE` |
| `OK <text>` | `CONNECT` (`connected`), `QUIT` (`bye`), `TALK` (dialogue line) |
| `OK <key>=<value>` | `MOVE` (`room=`), `WHO` (`players=`), `TAKE` (`taken=`), `DROP` (`dropped=`), `GROUP` (`group=`), `GOLD` (`gold=`) |
| `OK <json>` | `LOOK`, `INVENTORY`, `STATUS`, `ATTACK`, `QUEST`, `QUESTS`, and the extension queries |

Deviation: **`Payload::Pair` is a `HashMap`, not a single pair.** The RFC only ever shows one `key=value` per response, but `BUY`, `SELL` and `DICES` need to report two facts at once (e.g. the item and the remaining gold), so they emit several space-separated pairs on one line. The grammar stays `key=value` tokens; only the cardinality changes. Note that `HashMap` iteration order is unspecified, so **clients must parse pairs by key, never by position**.

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
- **Retrieve** and deliver an item to an NPC
- **Collect** specific items
- **Talk** to an NPC
- **Answer** a riddle.

The player structure has two attributes for quest management: 
- **quests_in_progress**: a HashMap with the quest ID as key and its progress step as value
- **finished_quest**: a HashSet containing the IDs of completed quests.

When a player accepts a quest from an NPC, if the quest ID is neither in quests_in_progress nor in finished_quests, it is added to quests_in_progress with a step value of 0.

After each player request, at the end of the handle_request function, the progress of active quests is evaluated.

If the request satisfies a quest goal, the quest is updated in quests_in_progress. The step is incremented by one, and an update event is sent to the client.

If the current step equals the total number of goals, the quest is marked as finished, and a completion event is sent to the client.

The reward is then added to the player's inventory, and the quest ID is moved to finished_quests.

# World Design

## This is our world:

![Our beautiful world](/images/image.png "City")

The central part is The Answer Place at the center of the map, the default spawn room.

## NPCS

There are three different types of NPC in this world: 
- **Citizen** : Just some dialogs (``TALK`` command).
- **Merchant** : You can shop with him (``BUY`` or ``SELL`` command) or just talk to him.
- **Enemy** : You can fight him (``ATTACK`` command) or just talk to him.

Every NPC may have a quest for you (``QUEST`` command), if you want to be sure you can use the ``NPC <id_npc>`` command.

## Items

There are four types of items in this world:
- **Armor** : Increase your armor value. The armor value is subtracted from the damage you take in combat.
- **Weapon** : You have a basic attack value of 15. If you have a weapon, you deal damage equal to the weapon's damage value.
- **Potion** : During a fight (or not) you can drink a potion (it's not really potions, it's mostly just food) to healing you.
- **Miscellaneous** : Often a quest item or just gold.

## Dungeons

In the Forest (The room on the far right of the map) we can create a dungeon (with the DUNGEON CREATE command on CLI or simply enter the dungeon using the GUI).

Dungeons are procedurally generated using the names of existing rooms. There will be between three and six rooms per dungeon.

The rooms are populated exclusively with enemies from those already included in the configuration.

The created dungeon is linked with a group. If the player is not in one, a group is automatically created.

All members of the group can join the dungeon with the DUNGEON JOIN command.

A dungeon is removed when the group is deleted (all members leaved it or leaved the game) or all enemies or defeated.

You can leave a dungeon by taking the same path you used to enter it but the dungeon is not removed while the above criteria aren't satisfied.


# Server Logging

Logging is built on `tracing` + `tracing-subscriber`, initialised once in `server::run` (`server/src/lib.rs`). We chose `tracing` over `log` for one reason: a MUD server interleaves dozens of clients in a single log stream, so **every line has to carry who it belongs to**. `tracing`'s spans attach that context automatically instead of requiring every call site to remember to print the address.

## Output destinations

Two layers are installed on the same subscriber, so every event is emitted twice, in two different shapes:

| Destination | Format | Purpose |
|---|---|---|
| `stdout` | human-readable (`tracing_subscriber::fmt`) | live monitoring while the server runs |
| `logs/tap.log.<YYYY-MM-DD>` | one JSON object per line | post-mortem analysis, greppable/queryable with `jq` |

The file layer writes through `tracing_appender::non_blocking`, so log writes happen on a dedicated thread and a slow disk never stalls a connection task, and through `rolling::daily`, so files rotate per day without any external logrotate. `logs/` is git-ignored.

## Log level

The level is driven by the standard `RUST_LOG` environment variable via `EnvFilter`, defaulting to `info` when it is unset:

```BASH
cargo run -p server server/config.yaml              # info
RUST_LOG=debug cargo run -p server server/config.yaml   # + lifecycle traces
RUST_LOG=server::handlers::fight=debug,info cargo run -p server server/config.yaml   # per-module
```

`info` is the operational level: connections, commands, responses, and game facts. `debug` adds the lifecycle noise (TCP open/close, world save, reset, shutdown) that is only useful when diagnosing the server itself.

## Log format

Each JSON line carries a timestamp, a level, the message and its structured fields, the emitting module (`target`), and the enclosing span:

```json
{
    "timestamp": "2026-07-28T05:56:07.420433Z",
    "level": "INFO",
    "fields": {
        "message": "command received",
        "command": "connect",
        "params": "[\"remi\"]"
    },
    "target": "server",
    "span": { 
            "peer_addr":"127.0.0.1:49098",
            "name": "connection"
    },
    "spans": [
        {
            "peer_addr": "127.0.0.1:49098",
            "name": "connection"
        }
    ]
}
```

The key mechanism is the **`connection` span**, opened at `accept()` and attached to the whole client task with `.instrument(span)`. It is created with `peer_addr` filled and `player` empty; `CONNECT` then back-fills it with `tracing::Span::current().record("player", name)`. From that point on, **every line produced anywhere in that task — including deep inside a handler that has no idea what a socket is — carries both the address and the player name**, without a single call site passing them around.

## Event types

| Category | Level | Emitted where | Fields |
|---|---|---|---|
| Command received | `info` | read loop | `command`, `params` |
| Response sent, success | `info` | read loop | `code=0` |
| Response sent, game error (`< 900`) | `info` | read loop | `code`, `error` |
| Response sent, protocol error (`>= 900`) | **`warn`** | read loop | `code`, `error` |
| Connection lifecycle | `debug` / `error` | `lib.rs` | TCP established / closed, write failure |
| Player actions | `info` | handlers | connect, move (`form`, `to`), take/drop (`item`), chat (`scope`, `body`), talk (`npc`) |
| Group lifecycle | `info` | `state/group.rs` | created, joined, left, deleted (`group`, `player`) |
| Combat | `info` | `handlers/fight/` | attack landed, enemy defeated (`attacker`, `target`, `damage`, `enemy_hp`, `loot`), player defeated |
| Quests | `info` | `state/quest.rs` | quest progressed (`quest`, `step`), quest completed (`quest`, `reward`) |
| Dungeons | `info` | `state/dungeon.rs` | dungeon cleared / deleted (`dungeon`) |
| Server lifecycle | `debug` | `lib.rs` | reset started/done, world saved, players saved, shutting down |

The **`900+` codes are logged at `warn` while game errors stay at `info`** — that is the single most useful decision in this whole section. A player walking into a wall (`301 NO_EXIT`) is normal gameplay and belongs at `info`; a client sending an unparsable verb (`903`) or a duplicate connect (`904`) means either a broken client or someone poking at the socket by hand. Filtering on `WARN` therefore surfaces exactly the anomalous traffic, with zero gameplay noise.

## Monitoring server behaviour

Live:

```BASH
cargo run -p server server/config.yaml            # human-readable stream on stdout
```

Per-player replay — the span makes it a one-liner:

```BASH
jq -r 'select(.span.player == "remi")' logs/tap.log.2026-07-28
```

Command mix and error distribution:

```BASH
jq -r 'select(.fields.message == "command received") | .fields.command' logs/*.log.* | sort | uniq -c | sort -rn
jq -r 'select(.fields.error) | .fields.error' logs/*.log.* | sort | uniq -c | sort -rn
```

## Detecting abuse patterns

The server does not implement rate limiting; the logs are the detection layer, and every field needed for it is already indexed by the `connection` span.

**Protocol probing / broken client** — a peer producing a burst of `9xx`:

```BASH
jq -r 'select(.level == "WARN") | .span.peer_addr' logs/tap.log.$(date +%F) \
  | sort | uniq -c | sort -rn | head

# Group Contributions
# Building and Running
# Testing
