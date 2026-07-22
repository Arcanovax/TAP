use crate::structures::dungeon::{Dungeon, format_dungeon_id};
use crate::{
    protocol::{Message, Payload},
    state::{ServerInfo, SharedServer, Tx},
    structures::{
        enums::{error::ErrorCode, exits::Direction, item_kind::ItemKind, npc_kind::NPCKind},
        game::World,
        item::Item,
        npc::NPC,
        quest::{Goal, Quest},
        room::Room,
    },
};
use redb::{Database, backends::InMemoryBackend};
use std::collections::HashMap;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use uuid::Uuid;

pub(crate) fn test_server() -> SharedServer {
    let db = Arc::new(test_db());
    Arc::new(Mutex::new(ServerInfo::new(World::new(), db)))
}

pub(crate) fn addr(n: u16) -> SocketAddr {
    ([127, 0, 0, 1], n).into()
}

pub(crate) fn connect(
    server: &SharedServer,
    addr: SocketAddr,
    name: &str,
) -> UnboundedReceiver<Message> {
    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    server
        .lock()
        .unwrap()
        .try_add_player(name.to_string(), addr, &tx)
        .expect("test connection");
    rx
}

pub(crate) fn err(code: ErrorCode) -> Message {
    Message::Response {
        error: code,
        payload: Payload::Empty,
    }
}

pub(crate) fn group_with(
    server: &SharedServer,
    members: &[(SocketAddr, &str)],
) -> Vec<UnboundedReceiver<Message>> {
    if members.len() < 2 {
        panic!("Not enough members to create a significant group");
    }

    let mut rxs = Vec::new();

    for (i, member) in members.iter().enumerate() {
        rxs.push(connect(server, member.0, member.1));
        if i == 0 {
            server
                .lock()
                .unwrap()
                .try_create_group(member.0, "test group")
                .expect("Group creation failed");
        } else {
            server
                .lock()
                .unwrap()
                .try_invite_group(member.1.to_string(), members[0].0)
                .expect("Failed to invite");
            server
                .lock()
                .unwrap()
                .try_join_group(member.0, members[0].1.to_string())
                .expect("Failed to join group");
        }
    }

    for rx in &mut rxs {
        while rx.try_recv().is_ok() {}
    }
    rxs
}

/// Réponse SUCCESS portant un payload JSON (cas des handlers qui renvoient du JSON).
pub(crate) fn ok_data(data: &str) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(
            serde_json::from_str(data).expect("ok_data: littéral JSON invalide"),
        ),
    }
}

/// Réponse SUCCESS au format `key=value`, une ou plusieurs paires
/// (ex: `ok_pair(&[("room", "loc.x")])` ou `ok_pair(&[("bought", "sword"), ("amount", "1")])`).
pub(crate) fn ok_pair(pairs: &[(&str, &str)]) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        ),
    }
}

/// Réponse SUCCESS au format texte simple (ex: `OK connected`).
pub(crate) fn ok_text(text: &str) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Text(text.to_string()),
    }
}

/// Un canal nu, pour les handlers qui prennent un `&Tx` (ex. connect).
pub(crate) fn tx_rx() -> (Tx, UnboundedReceiver<Message>) {
    mpsc::unbounded_channel::<Message>()
}

/// Vérifie une réponse SUCCESS dont la `data` contient `needle`.
/// Utile quand le JSON sérialisé n'est pas déterministe (HashMap/HashSet).
pub(crate) fn assert_success_contains(msg: &Message, needle: &str) {
    let wire = msg.to_str();
    assert!(
        wire.starts_with("OK"),
        "expected an OK response, got {wire:?}"
    );
    assert!(
        wire.contains(needle),
        "wire {wire:?} does not contain {needle:?}"
    );
}

/// Extrait le code d'erreur d'une réponse (utile quand la `data` n'est pas vide,
/// donc qu'on ne peut pas comparer le `Message` entier avec `err`).
pub(crate) fn response_error(msg: &Message) -> &ErrorCode {
    match msg {
        Message::Response { error, .. } => error,
        other => panic!("expected a Message::Response, got {other:?}"),
    }
}

/// Un `World` peuplé pour les handlers qui dépendent du monde.
/// - room `room.city_square` (= location par défaut d'un joueur) avec `guard`, `goblin`, `merchant`,
///   `sword` et une sortie Nord vers `room.market`
/// - room `room.market` (sortie Sud retour vers `room.city_square`)
/// - npc `guard` (Citizen, porteur de `quest.fetch`), `goblin` (Enemy non vaincu),
///   `merchant` (Merchant, vend `sword`, 100 gold), `villager` (Citizen sans quête)
/// - item `sword` (prix 10), quête `quest.fetch`
pub(crate) fn test_world() -> World {
    let mut world = World::new();

    world.rooms.insert(
        "room.city_square".to_string(),
        Room {
            name: "room.city_square".to_string(),
            exits: HashMap::from([(Direction::North, "room.market".to_string())]),
            description: "The city square".to_string(),
            npc: vec![
                "guard".to_string(),
                "goblin".to_string(),
                "merchant".to_string(),
            ],
            items: vec!["sword".to_string().into()],
        },
    );

    world.rooms.insert(
        "room.market".to_string(),
        Room {
            name: "room.market".to_string(),
            exits: HashMap::from([(Direction::South, "room.city_square".to_string())]),
            description: "The market".to_string(),
            npc: Vec::new(),
            items: Vec::new(),
        },
    );

    world.items.insert(
        "sword".to_string(),
        Item {
            name: "sword".to_string(),
            price: 10,
            kind: ItemKind::Miscellaneous,
        },
    );

    world.items.insert(
        "quest_item".to_string(),
        Item {
            name: "quest_item".to_string(),
            price: 10,
            kind: ItemKind::QuestItem,
        },
    );

    world.npcs.insert(
        "guard".to_string(),
        NPC {
            name: "guard".to_string(),
            dialog: HashMap::new(),
            kind: NPCKind::Citizen,
            quest: Some("quest.fetch".to_string()),
        },
    );
    world.npcs.insert(
        "merchant".to_string(),
        NPC {
            name: "merchant".to_string(),
            dialog: HashMap::new(),
            kind: NPCKind::Merchant {
                inventory: vec!["sword".to_string()],
            },
            quest: None,
        },
    );
    world.npcs.insert(
        "villager".to_string(),
        NPC {
            name: "villager".to_string(),
            dialog: HashMap::new(),
            kind: NPCKind::Citizen,
            quest: None,
        },
    );
    world.npcs.insert(
        "goblin".to_string(),
        NPC {
            name: "goblin".to_string(),
            dialog: HashMap::new(),
            kind: NPCKind::Enemy {
                hp: 30,
                max_hp: 30,
				kind: "goblin".to_string(),
                damages: 5,
                loot: Vec::new(),
                defeated: false,
            },
            quest: None,
        },
    );

    world.quests.insert(
        "quest.fetch".to_string(),
        Quest {
            name: "quest.fetch".to_string(),
            description: "Fetch a sword".to_string(),
            reward: "gold".to_string(),
            goals: vec![Goal::Collect {
                item: "sword".to_string(),
                amount: 1,
            }],
        },
    );
    world.spawn_room = "room.city_square".to_string();
    world.gambling_room = "room.city_quare".to_string();
    world.dungeon_entrance = "room.city_square".to_string();

    world
}

/// Serveur dont le monde est peuplé par [`test_world`].
pub(crate) fn populated_server() -> SharedServer {
    let db = Arc::new(test_db());
    Arc::new(Mutex::new(ServerInfo::new(test_world(), db)))
}

pub(crate) fn test_db() -> Database {
    Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .expect("in-memory test db failed to create")
}

pub fn give_gold(server: &crate::state::SharedServer, addr: std::net::SocketAddr, gold: u32) {
    server.lock().unwrap().get_player_mut(addr).unwrap().gold = gold;
}

/// Place `amount` exemplaires de `item` dans l'inventaire du joueur.
pub fn give_item(
    server: &crate::state::SharedServer,
    addr: std::net::SocketAddr,
    item: &str,
    amount: u32,
) {
    server
        .lock()
        .unwrap()
        .get_player_mut(addr)
        .unwrap()
        .inventory
        .insert(item.to_string(), amount);
}

// ---------------------------------------------------------------------------
// Fixtures « donjon »
//
// `generate_dungeon` étant aléatoire, on construit ici un donjon *déterministe*
// à gid fixe, calqué sur la forme de [`test_world`] pour que les assertions
// soient stables. Les entités portent leurs ids de donjon (`*.dg_{gid}_{n}`) ;
// les handlers ne résolvant le nom→id que via `world.name_to_ref`, les tests
// « donjon » référencent les entités par leur id brut (voir [`dg_item`], etc.).
// ---------------------------------------------------------------------------

/// Le gid fixe partagé par toutes les fixtures de donjon.
pub(crate) fn test_gid() -> Uuid {
    Uuid::from_u128(1)
}

/// Id de la `n`-ième room du donjon de test.
pub(crate) fn dg_room(n: u8) -> String {
    format_dungeon_id("room", test_gid(), n as usize)
}

/// Id du `n`-ième item du donjon de test.
pub(crate) fn dg_item(n: u8) -> String {
    format_dungeon_id("item", test_gid(), n as usize)
}

/// Id du `n`-ième npc du donjon de test.
pub(crate) fn dg_npc(n: u8) -> String {
    format_dungeon_id("npc", test_gid(), n as usize)
}

/// Un `Dungeon` déterministe à deux salles :
/// - entrée `dg_room(0)` : item `dg_item(0)` (= « sword », prix 10), ennemi
///   `dg_npc(0)` (= « goblin » non vaincu), sortie Nord vers `dg_room(1)`
/// - `dg_room(1)` : vide, sortie Sud retour vers l'entrée
pub(crate) fn test_dungeon() -> Dungeon {
    let mut rooms = HashMap::new();
    rooms.insert(
        dg_room(0),
        Room {
            name: dg_room(0),
            exits: HashMap::from([(Direction::North, dg_room(1))]),
            description: "The dungeon entrance".to_string(),
            npc: vec![dg_npc(0)],
            items: vec![dg_item(0).into()],
        },
    );
    rooms.insert(
        dg_room(1),
        Room {
            name: dg_room(1),
            exits: HashMap::from([(Direction::South, dg_room(0))]),
            description: "The dungeon depths".to_string(),
            npc: Vec::new(),
            items: Vec::new(),
        },
    );

    let mut npcs = HashMap::new();
    npcs.insert(
        dg_npc(0),
        NPC {
            name: "goblin".to_string(),
            dialog: HashMap::new(),
            kind: NPCKind::Enemy {
                hp: 30,
                max_hp: 30,
				kind: "goblin".to_string(),
                damages: 5,
                loot: Vec::new(),
                defeated: false,
            },
            quest: None,
        },
    );

    let mut items = HashMap::new();
    items.insert(
        dg_item(0),
        Item {
            name: "sword".to_string(),
            price: 10,
            kind: ItemKind::Miscellaneous,
        },
    );

    Dungeon { rooms, npcs, items }
}

/// [`populated_server`] auquel on a ajouté le donjon déterministe [`test_dungeon`].
pub(crate) fn dungeon_server() -> SharedServer {
    let server = populated_server();
    server
        .lock()
        .unwrap()
        .dungeons
        .insert(test_gid(), test_dungeon());
    server
}

/// Comme [`connect`], mais place immédiatement le joueur dans l'entrée du donjon.
pub(crate) fn connect_in_dungeon(
    server: &SharedServer,
    addr: SocketAddr,
    name: &str,
) -> UnboundedReceiver<Message> {
    let rx = connect(server, addr, name);
    {
        let mut guard = server.lock().unwrap();
        let player = guard.get_player_mut(addr).unwrap();
        player.location = dg_room(0);
        // Le donjon est indexé par le `group_id` du joueur (cf. `try_create_dungeon`).
        // Sans ça, les handlers qui résolvent via `group_id` retombent sur le world.
        player.group_id = Some(test_gid());
    }
    rx
}
