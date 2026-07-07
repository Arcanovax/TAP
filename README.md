*This project has been created as part of the 42 curriculum by bfitte, mthetcha and relaforg.*

# The Answer Protocol

## Description

**TAP** (*The Answer Protocol*) is a multiplayer, shared-world retro text adventure.
Players connect to a central server over TCP and interact with a persistent world
through short text commands: they move between rooms, talk to NPCs, pick up and
trade items, take on quests, form groups, gamble, and fight enemies in turn-based
combat — including in procedurally generated dungeons.

The world is **shared**: every connected player acts in the same world at the same
time, sees each other's presence, chats by scope (global / group / room), and
receives live events pushed by the server (someone entering a room, an attack
landing, a quest update, a world reset, ...).

The server is written in **Rust** on top of the asynchronous **Tokio** runtime,
speaks a line-based text protocol (RFC 42TAP), and persists its state with
**redb**.

## Instructions

A Rust toolchain (edition 2024) is required. See **Building and Running** for
details and **Testing** for how to exercise the server.

## Resources

Classic references related to the topic:

- The Tokio documentation (async runtime, `tokio::select!`, channels) —
  <https://tokio.rs>
- The `tracing` / `tracing-subscriber` ecosystem for structured logging.
- `redb`, an embedded key-value store, for world and player persistence.
- The **RFC 42TAP** text-protocol specification (the wire format this server
  implements).

<!-- TODO(team): describe how AI was used — for which tasks (design specs,
     boilerplate refactors, tests, docs, ...) and which parts of the project.
     Be specific and honest; this section is explicitly graded. -->

---

## Architecture

**Concurrency model**
- One `tokio::spawn`ed task **per TCP connection**.
- Shared world state is a single `Arc<Mutex<ServerInfo>>` (`SharedServer`);
  handlers lock it for the duration of a command. State mutation is therefore
  serialized — chosen for simplicity and correctness over fine-grained locking.
- Each connection owns an **unbounded MPSC channel** (`Tx`). Other tasks push
  server-initiated `Message::Event`s into it. The connection loop uses
  `tokio::select!` to multiplex two sources: lines read from the client, and
  events received on the channel — both are written back to the same socket.

**Dispatcher (router, not inline handling)**
- `handlers/handle_request.rs` is the central dispatcher. It parses the command
  name into a `Command` enum (`strum`, case-insensitive) and routes to a
  dedicated handler function per command (one module per handler under
  `handlers/`).
- A pre-check rejects world-mutating commands (`TAKE`, `DROP`, `QUEST`, `BUY`,
  `SELL`, `TALK`, `MOVE`) while the player is `InFight` (`FORBIDDEN_ACTION`).
- After every command, `advance_quests` runs globally, fed with an optional
  `GameEvent` produced by the handler.

**Background & lifecycle**
- A background task ticks every **600 s** to reset the world (respawn enemies,
  restore merchant stock, restore config items) and persist it to redb.
- `SIGINT` / `SIGTERM` trigger a graceful shutdown that saves the world and all
  connected players.

**Persistence**
- `redb` (`game.redb`) with `bincode` encoding, split across `persistence/`
  (players, world, tables). The YAML config is the structural authority; the
  database carries player-dropped items merged on top at startup.

## Protocol Implementation

The server implements **RFC 42TAP**, a line-based text protocol. Framing:

- `OK [payload]` — success. Payload is `Empty`, `Text`, `key=value` `Pair`, or
  inline `Json` depending on the command.
- `ERR <code> <NAME>` — failure (e.g. `ERR 201 NAME_IN_USE`). No payload on error.
- `EVT <category> <type> <data>` — server-pushed events.
- Greeting on connect: `OK hello proto=1`.

The full conformance analysis (framing rules, per-command payload mapping, JSON
payload shapes for `LOOK`/`INVENTORY`/`STATUS`/`ATTACK`/`QUEST(S)`) is documented
in `server/docs/superpowers/specs/2026-06-24-rfc-text-protocol-design.md`.

**Deviations / extensions from the RFC** (kept RFC-compatible by reusing the same
framing):

- **Extra commands** beyond the core RFC: `BUY`, `SELL`, `GOLD`, `DUNGEON`,
  `SLOT_MACHINE`, `DICES`, `ROOM`, `ROOMS`, `NPC`, `NPCS`, `ITEM`, `ITEMS`,
  `QUEST_INFO`, `HELP`.
- **Extra events**: `QUEST UPDATE` / `QUEST FINISH` (JSON blob to avoid space
  ambiguity in quest names), `ROOM TAKE` / `ROOM DROP`, and the `FIGHT *` family
  (`ENTER`, `ATTACK`, `ENEMY`, `HEALING`, `LEAVE`).
- **Extra error codes**: gameplay codes (`NOT_ENOUGH_GOLD` 408, `GAME_LOSE` 409,
  dungeon codes 410/411) and internal codes in the `9xx` range
  (`CONNECTION_FAILED`, `SEND_FAILED`, `INVALID_ARGS`, `INVALID_COMMAND`, ...).
- Explanatory text that a non-conformant version used to attach to `ERR` payloads
  was removed to stay RFC-pure.

## Combat System

Combat is **turn-based** and can be solo or cooperative (a whole group can join
the same fight).

**Damage formula**
- A player deals a base **15** damage, upgraded to the highest `damages` value
  among the `Weapon` items in their inventory.
- Enemies (`NPCKind::Enemy`) carry `hp` / `max_hp` / fixed `damages` / `loot`.
- `Armor` (`protection`) and `Potion` (`healing`, via `CONSUME`) items exist for
  survivability; players start at **100 HP**.

**Initiative / turn order**
- A `Fight` holds an ordered `fighters` list and a `turn` index. Players act in
  list order; when the last fighter has acted, the turn wraps and the **enemy
  attacks**, then the index resets to 0.
- Multi-fighter target selection is pseudo-random (`nanos % nb_fighters`).

**Death & rewards**
- If an enemy attack would drop a player to 0, the player is defeated: respawned
  at `room.city_square` with `max_hp - 10` HP, status back to `Idle`.
- On enemy kill, `loot` is distributed to every fighter; `item.gold` converts to
  **+50 gold**, other loot is added to the inventory.
- Clearing every enemy in a dungeon closes it (`dungeon cleared`).

**Combat commands**
- `ATTACK <target>` — start or continue a fight.
- `FLEE <target>` — leave the fight, at a cost of **10 gold**.
- `CONSUME <item>` — use a potion (heal) mid-fight.

> <!-- TODO(team): confirm/adjust this justification if DEFEND is added later. -->

## Quest System

Quests are defined in the room YAML config. A `Quest` has a `name`, `description`,
`reward` (an item id) and an ordered list of `goals`.

**Goal types** (`structures/quest.rs`)
- `Collect { item, amount }` — satisfied when the inventory holds enough.
- `Talk { dialog }` — satisfied by triggering a specific NPC dialogue line.
- `Retrieve { item, amount, dialog }` — hand items to an NPC: requires both the
  items **and** the dialogue, and **consumes** the items.

**Progression & validation**
- A player accepts a quest with `QUEST <npc>` (the NPC must be in the room and
  offer a quest). Progress is tracked per player as a goal index in
  `quests_in_progress`.
- After **every** command, `advance_quests` re-evaluates the current goal against
  the player state and an optional `GameEvent` (e.g. a `Talked` event), advancing
  one goal at a time and emitting `EVT QUEST UPDATE`.
- Finishing the last goal grants the `reward` and emits `EVT QUEST FINISH`;
  completed quests are recorded in `finished_quest`.

## World Design

**Static world** (`server/config.yaml` + `server/config/`)
- The root config declares the `spawn_point` (`room.city_square`), the
  `gambling_room` (`room.game_room`), the `dungeon_entrance` (`room.forest`), and
  imports the room and item definitions.
- **11 hand-authored rooms**: city square, tavern, game room, farm, market, beach,
  parc, blacksmith, forest, quarry, mine — connected by `North/South/East/West`
  exits.
- **NPC roles** (`NPCKind`): `Citizen` (dialogue + quest givers), `Merchant`
  (buy/sell inventory), `Enemy` (combat, with loot).
- **Items** (`ItemKind`): `Weapon { damages }`, `Armor { protection }`,
  `Potion { healing }`, `Miscellaneous`, distributed across rooms and merchant
  stock. New players spawn with **100 HP** and **50 gold**.

**Procedural dungeons** (`dungeon/generation.rs`)
- Entered from the dungeon entrance; generated on a coordinate grid of **3–6
  rooms** linked in the four cardinal directions, with the start room linking West
  back to the overworld.
- Each room is populated with **1–3 enemies** and **1–2 items** drawn from the
  world pools. Dungeon entities use namespaced ids (per-instance `Uuid`) so they
  never collide with the shared world.

## Server Logging

Logging uses the **`tracing`** ecosystem, configured in `lib.rs`:

- **Two layers**: a human-readable `fmt` layer to stdout, and a **JSON** layer
  written to `logs/tap.log`, rotated **daily** via `tracing-appender` on a
  non-blocking writer.
- **Filtering**: `EnvFilter` from `RUST_LOG`, defaulting to `info`.
- **Spans**: each connection runs inside an `info_span!("connection", peer_addr,
  player)` so every log line is attributable to a client (and player once known).
- **Event types**: `command received` (name + params), `response sent`
  (`code`, and `warn` for internal `9xx` codes), combat events (`attack landed`,
  `enemy defeated`, `player defeated by enemy`), `quest accepted`,
  `dungeon cleared`, connect/disconnect, world save/reset.

**Monitoring & abuse patterns**
- The structured JSON log (one object per line) is meant to be tailed/ingested;
  `code` and symbolic error names make it easy to alert on bursts of `9xx`
  (protocol abuse), repeated `NAME_IN_USE`, etc.
<!-- TODO(team): document any additional monitoring/alerting you rely on. -->

## Group Contributions

This project was built by **bfitte**, **mthetcha** and **relaforg**.

<!-- TODO(team): fill in each member's responsibilities and contributions per
     component. Example structure:
     - bfitte   — ...
     - mthetcha — ...
     - relaforg — ...
     (server, CLI client, GUI client, world design, protocol, combat, quests,
     persistence, logging, tests, ...) -->

## Building and Running

**Requirements**: a Rust toolchain with **edition 2024** support and Cargo.

**Server**

```bash
cd server
cargo build --release      # build
cargo run                  # run (listens on 127.0.0.1:8080)
RUST_LOG=debug cargo run   # run with verbose logging
```

State is persisted to `server/game.redb`; logs are written to `server/logs/`.

**Clients**

<!-- TODO(team): document the CLI client and GUI client here (build tool + run
     command for each). They are NOT part of this `server` branch yet — only a
     Python integration script (`server/test.py`) currently exercises the server.
     Update this section once the clients land, or point to their repositories. -->

## Testing

**Rust unit / integration tests**

```bash
cd server
cargo test
```

Handlers are tested in sibling `tests.rs` modules (combat, quests, movement,
inventory, groups, protocol encoding golden tests, ...). Protocol conformance is
covered by exact-string assertions per `Payload` / `EventType` variant in
`protocol.rs`.

<!-- TODO(team): describe your multiplayer test scenarios (concurrent players,
     group fights, quest completion end-to-end) and how to reproduce them. -->
