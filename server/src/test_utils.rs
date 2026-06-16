use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, UnboundedReceiver};

use std::collections::HashMap;

use crate::{
    error::ErrorCode,
    game::World,
    protocol::Message,
    state::{ServerInfo, SharedServer, Tx},
    structures::{
        enums::{exits::Exit, item_kind::ItemKind, npc_kind::NPCKind},
        item::Item,
        npc::NPC,
        quest::{Goal, Quest},
        room::Room,
    },
};

pub(crate) fn test_server() -> SharedServer {
    Arc::new(Mutex::new(ServerInfo::new(World::new())))
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
        data: None,
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
                .try_join_group(member.0)
                .expect("Failed to join group");
        }
    }

    for rx in &mut rxs {
        while rx.try_recv().is_ok() {}
    }
    rxs
}

/// Réponse SUCCESS portant une `data` déjà sérialisée (cas des handlers qui renvoient du JSON).
pub(crate) fn ok_data(data: &str) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::from_str(data).expect("ok_data: littéral JSON invalide")),
    }
}

/// Un canal nu, pour les handlers qui prennent un `&Tx` (ex. connect).
pub(crate) fn tx_rx() -> (Tx, UnboundedReceiver<Message>) {
    mpsc::unbounded_channel::<Message>()
}

/// Vérifie une réponse SUCCESS dont la `data` contient `needle`.
/// Utile quand le JSON sérialisé n'est pas déterministe (HashMap/HashSet).
pub(crate) fn assert_success_contains(msg: &Message, needle: &str) {
    match msg {
        Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(d),
        } => assert!(
            d.to_string().contains(needle),
            "data {d:?} does not contain {needle:?}"
        ),
        other => panic!("expected SUCCESS with data containing {needle:?}, got {other:?}"),
    }
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
/// - room `room.city_square` (= location par défaut d'un joueur) avec `guard`, `goblin`, `sword`
///   et une sortie Nord vers `room.market`
/// - room `room.market` (sortie Sud retour vers `room.city_square`)
/// - npc `guard` (Citizen, porteur de `quest.fetch`), `goblin` (Enemy non vaincu), `villager` (Citizen sans quête)
/// - item `sword`, quête `quest.fetch`
pub(crate) fn test_world() -> World {
    let mut world = World::new();

    world.rooms.insert(
        "room.city_square".to_string(),
        Room {
            name: "room.city_square".to_string(),
            exits: vec![Exit::North {
                toward: "room.market".to_string(),
            }],
            description: "The city square".to_string(),
            npc: vec!["guard".to_string(), "goblin".to_string()],
            items: vec!["sword".to_string()],
        },
    );

    world.rooms.insert(
        "room.market".to_string(),
        Room {
            name: "room.market".to_string(),
            exits: vec![Exit::South {
                toward: "room.city_square".to_string(),
            }],
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

    world
}

/// Serveur dont le monde est peuplé par [`test_world`].
pub(crate) fn populated_server() -> SharedServer {
    Arc::new(Mutex::new(ServerInfo::new(test_world())))
}
