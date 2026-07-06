use super::*;
use crate::structures::room::Owner;
use std::fs;
use std::path::PathBuf;

/// Répertoire temporaire jetable : `load` lit et `canonicalize` sur le disque,
/// donc on ne peut pas tester en mémoire — il faut de vrais fichiers.
/// Le `Drop` nettoie, même en cas de panic d'une assertion.
struct TmpDir {
    root: PathBuf,
}

impl TmpDir {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("tap-config-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&root).unwrap();
        TmpDir { root }
    }

    /// Écrit un fichier (relatif à la racine du tmp) et rend son chemin absolu.
    fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.root.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, content).unwrap();
        path
    }
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

// ---------------------------------------------------------------------------
// Chemins heureux
// ---------------------------------------------------------------------------

#[test]
fn load_minimal_world_is_ok() {
    // Le plus petit monde valide : une room, et les TROIS singletons pointant
    // dessus (cf. check_singles_type qui les exige tous les trois).
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let world = load(&entry).expect("le monde minimal doit charger");
    assert_eq!(world.spawn_room, "room.hub");
    assert_eq!(world.gambling_room, "room.hub");
    assert_eq!(world.dungeon_entrance, "room.hub");
    assert!(world.rooms.contains_key("room.hub"));
}

#[test]
fn load_populates_all_collections_and_prefixes_ids() {
    // Un fichier riche : room + npc (avec dialog) + item + quest, toutes les
    // références résolues localement. Vérifie le préfixage d'id et la
    // résolution d'une référence de dialog (npc.guard.dialog.greet).
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"

item:
  gem:
    name: "Gem"
    price: 5
    kind: Miscellaneous

npc:
  guard:
    name: "Guard"
    dialog:
      greet:
        - "Hello"
    kind: Citizen
    quest: "quest.find"

quest:
  find:
    name: "Find"
    description: "Find the gem"
    reward: "item.gem"
    goals:
      - Collect:
          item: "item.gem"
          amount: 1
      - Talk:
          dialog: "npc.guard.dialog.greet"

room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
    npc:
      - "npc.guard"
    items:
      - "item.gem"
"#,
    );

    let world = load(&entry).expect("le monde riche doit charger");

    assert!(world.rooms.contains_key("room.hub"));
    assert!(world.npcs.contains_key("npc.guard"));
    assert!(world.quests.contains_key("quest.find"));

    let item = world.items.get("item.gem").expect("item.gem présent");
    assert_eq!(item.name, "Gem");
    assert_eq!(item.price, 5);

    // Un item posé dans une room appartient à la Room par défaut.
    let hub = &world.rooms["room.hub"];
    assert_eq!(hub.items[0].item, "item.gem");
    assert_eq!(hub.items[0].owner, Owner::Room);
}

#[test]
fn load_resolves_references_across_imported_files() {
    // La spawn room est définie dans un fichier importé, l'id est bien visible.
    let dir = TmpDir::new();
    dir.write(
        "world.yaml",
        r#"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );
    let entry = dir.write(
        "entry.yaml",
        r#"
import:
  - "world.yaml"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
"#,
    );

    let world = load(&entry).expect("import doit charger");
    assert!(world.rooms.contains_key("room.hub"));
}

#[test]
fn diamond_imports_are_loaded_only_once() {
    // A -> B, C ; B -> D ; C -> D. D est atteignable par deux chemins.
    // Sans le dedup via `visited`, item.gem serait défini deux fois -> Conflict.
    let dir = TmpDir::new();
    dir.write(
        "d.yaml",
        r#"
item:
  gem:
    name: "Gem"
    price: 1
    kind: Miscellaneous
"#,
    );
    dir.write("b.yaml", "import:\n  - \"d.yaml\"\n");
    dir.write("c.yaml", "import:\n  - \"d.yaml\"\n");
    let entry = dir.write(
        "entry.yaml",
        r#"
import:
  - "b.yaml"
  - "c.yaml"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let world = load(&entry).expect("import en diamant ne doit pas dupliquer");
    assert!(world.items.contains_key("item.gem"));
}

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

#[test]
fn missing_spawn_point_is_reported() {
    // Aucun spawn_point : erreur retournée avant même les checks de refs.
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::MissingSpawnPoint));
}

#[test]
fn missing_entry_file_is_io_error() {
    let dir = TmpDir::new();
    let missing = dir.root.join("does_not_exist.yaml");

    let err = load(&missing).unwrap_err();
    assert!(matches!(err, ConfigError::Io { .. }));
}

#[test]
fn missing_imported_file_is_io_error() {
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
import:
  - "ghost.yaml"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
"#,
    );

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::Io { .. }));
}

#[test]
fn invalid_yaml_is_parse_error() {
    // `import` attend une séquence, on lui donne un entier -> erreur serde.
    let dir = TmpDir::new();
    let entry = dir.write("entry.yaml", "import: 42\n");

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::Parse { .. }));
}

#[test]
fn duplicate_id_across_files_is_conflict() {
    // room.hub est défini dans l'entry ET dans le fichier importé.
    let dir = TmpDir::new();
    dir.write(
        "dup.yaml",
        r#"
room:
  hub:
    name: "Hub dup"
    description: "d"
    exits: {}
"#,
    );
    let entry = dir.write(
        "entry.yaml",
        r#"
import:
  - "dup.yaml"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
room:
  hub:
    name: "Hub entry"
    description: "d"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    match err {
        ConfigError::Conflict { id, .. } => assert_eq!(id, "room.hub"),
        other => panic!("attendu Conflict, obtenu {other:?}"),
    }
}

#[test]
fn dangling_exit_reference_is_reported() {
    // Une sortie pointe vers une room jamais définie.
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
room:
  hub:
    name: "Hub"
    description: "central"
    exits:
      North: "room.void"
"#,
    );

    let err = load(&entry).unwrap_err();
    match err {
        ConfigError::DanglingRef { missing_ref, .. } => assert_eq!(missing_ref, "room.void"),
        other => panic!("attendu DanglingRef, obtenu {other:?}"),
    }
}

#[test]
fn reference_without_import_is_scope_violation() {
    // room.a (dans a.yaml) référence item.gem (dans b.yaml) mais a.yaml
    // n'importe pas b.yaml. L'id existe (check_refs passe) mais n'est pas
    // dans le scope visible depuis a.yaml -> ScopeViolation.
    let dir = TmpDir::new();
    dir.write(
        "a.yaml",
        r#"
room:
  a:
    name: "A"
    description: "d"
    exits: {}
    items:
      - "item.gem"
"#,
    );
    dir.write(
        "b.yaml",
        r#"
item:
  gem:
    name: "Gem"
    price: 1
    kind: Miscellaneous
"#,
    );
    let entry = dir.write(
        "entry.yaml",
        r#"
import:
  - "a.yaml"
  - "b.yaml"
spawn_point: "room.a"
gambling_room: "room.a"
dungeon_entrance: "room.a"
"#,
    );

    let err = load(&entry).unwrap_err();
    match err {
        ConfigError::ScopeViolation {
            from_id, ref_id, ..
        } => {
            assert_eq!(from_id, "room.a");
            assert_eq!(ref_id, "item.gem");
        }
        other => panic!("attendu ScopeViolation, obtenu {other:?}"),
    }
}

#[test]
fn singleton_pointing_to_non_room_is_wrong_ref() {
    // spawn_point pointe vers un item (id défini, donc check_refs/scope passent)
    // mais ce n'est pas une room -> WrongRef via check_singles_type.
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "item.gem"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
item:
  gem:
    name: "Gem"
    price: 1
    kind: Miscellaneous
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    match err {
        ConfigError::WrongRef { from_id, ref_id } => {
            assert_eq!(from_id, "spawn_point");
            assert_eq!(ref_id, "item.gem");
        }
        other => panic!("attendu WrongRef, obtenu {other:?}"),
    }
}

#[test]
fn missing_gambling_room_is_reported() {
    // gambling_room absent -> MissingGamblingRoom, retourné avant les checks
    // de refs (load() vérifie chaque singleton non vide dans l'ordre).
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::MissingGamblingRoom));
}

#[test]
fn missing_dungeon_entrance_is_reported() {
    // spawn_point et gambling_room présents, mais dungeon_entrance absent.
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
gambling_room: "room.hub"
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::MissingDungeonEntrance));
}

#[test]
fn dialog_id_colliding_with_npc_id_is_conflict() {
    // Un npc "hero" avec un dialog "greet" enregistre l'id
    // "npc.hero.dialog.greet". Un second npc littéralement nommé
    // "hero.dialog.greet" produit le même id -> Conflict.
    // (L'id exact remonté dépend de l'ordre d'itération de la HashMap, on
    // vérifie donc seulement la variante.)
    let dir = TmpDir::new();
    let entry = dir.write(
        "entry.yaml",
        r#"
spawn_point: "room.hub"
gambling_room: "room.hub"
dungeon_entrance: "room.hub"
npc:
  hero:
    name: "Hero"
    dialog:
      greet:
        - "hi"
    kind: Citizen
  "hero.dialog.greet":
    name: "Weird"
    kind: Citizen
room:
  hub:
    name: "Hub"
    description: "central"
    exits: {}
"#,
    );

    let err = load(&entry).unwrap_err();
    assert!(matches!(err, ConfigError::Conflict { .. }));
}
