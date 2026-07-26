use crate::{
    config::ConfigError,
    structures::{
        enums::exits::Direction,
        game::World,
        item::Item,
        npc::Npc,
        quest::Quest,
        room::{OwnedItem, Room},
    },
};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

const SPAWN_POINT: &str = "spawn_point";
const GAMBLING_TOOM: &str = "gambling_room";
const DUNGEON_ENTRANCE: &str = "dungeon_entrance";

#[derive(Deserialize, Debug)]
struct ConfigRoom {
    name: String,
    exits: HashMap<Direction, String>,
    description: String,
    #[serde(default)]
    npc: Vec<String>,
    #[serde(default)]
    items: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    #[serde(default)]
    import: Vec<String>,
    #[serde(default)]
    npc: HashMap<String, Npc>,
    #[serde(default)]
    item: HashMap<String, Item>,
    #[serde(default)]
    room: HashMap<String, ConfigRoom>,
    #[serde(default)]
    quest: HashMap<String, Quest>,
    #[serde(default)]
    spawn_point: String,
    #[serde(default)]
    gambling_room: String,
    #[serde(default)]
    dungeon_entrance: String,
}

#[derive(Debug)]
struct Loader {
    world: World,
    visited: HashSet<PathBuf>,
    definer: HashMap<String, PathBuf>,
    import_graph: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Loader {
    fn new() -> Self {
        Loader {
            world: World::new(),
            visited: HashSet::new(),
            definer: HashMap::new(),
            import_graph: HashMap::new(),
        }
    }

    fn define(&mut self, id: &str, path: &Path) -> Result<(), ConfigError> {
        if let Some(file_a) = self.definer.insert(id.to_string(), path.to_path_buf()) {
            return Err(ConfigError::Conflict {
                id: id.to_string(),
                file_a,
                file_b: path.to_path_buf(),
            });
        }
        Ok(())
    }

    fn referencing_entries(&self) -> impl Iterator<Item = (&str, Vec<&str>)> {
        let rooms = self
            .world
            .rooms
            .iter()
            .map(|(id, r)| (id.as_str(), r.references()));
        let npcs = self
            .world
            .npcs
            .iter()
            .map(|(id, n)| (id.as_str(), n.references()));
        let quests = self
            .world
            .quests
            .iter()
            .map(|(id, q)| (id.as_str(), q.references()));
        let singles = self
            .singletons()
            .into_iter()
            .filter(|(_, target)| !target.is_empty())
            .map(|(label, target)| (label, vec![target.as_str()]));

        rooms.chain(npcs).chain(quests).chain(singles)
    }

    fn singletons(&self) -> [(&str, &String); 3] {
        [
            (SPAWN_POINT, &self.world.spawn_room),
            (GAMBLING_TOOM, &self.world.gambling_room),
            (DUNGEON_ENTRANCE, &self.world.dungeon_entrance),
        ]
    }

    fn load_file(&mut self, path: &Path) -> Result<(), ConfigError> {
        let path = path.canonicalize().map_err(|source| ConfigError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        if self.visited.contains(&path) {
            return Ok(());
        }
        self.visited.insert(path.clone());

        let content = std::fs::read_to_string(&path).map_err(|source| ConfigError::Io {
            path: path.clone(),
            source,
        })?;
        let parsed =
            serde_yaml::from_str::<ConfigFile>(&content).map_err(|source| ConfigError::Parse {
                path: path.clone(),
                source,
            })?;

        for file in &parsed.import {
            let dep = path.parent().unwrap_or(Path::new(".")).join(file);
            let dep = dep.canonicalize().map_err(|source| ConfigError::Io {
                path: dep.clone(),
                source,
            })?;
            self.import_graph
                .entry(path.clone())
                .or_default()
                .push(dep.clone());
            self.load_file(&dep)?;
        }

        if !parsed.spawn_point.is_empty() {
            self.define(SPAWN_POINT, &path)?;
            self.world.spawn_room = parsed.spawn_point;
        }

        if !parsed.gambling_room.is_empty() {
            self.define(GAMBLING_TOOM, &path)?;
            self.world.gambling_room = parsed.gambling_room;
        }

        if !parsed.dungeon_entrance.is_empty() {
            self.define(DUNGEON_ENTRANCE, &path)?;
            self.world.dungeon_entrance = parsed.dungeon_entrance;
        }

        for (name, npc) in parsed.npc {
            let id = format!("npc.{}", name);
            self.define(&id, &path)?;
            for d in npc.dialog.keys() {
                if let Some(file_a) = self
                    .definer
                    .insert(id.clone() + ".dialog." + d, path.clone())
                {
                    return Err(ConfigError::Conflict {
                        id,
                        file_a,
                        file_b: path,
                    });
                }
            }
            self.world.npcs.insert(id.clone(), npc);
        }

        for (name, room) in parsed.room {
            let id = format!("room.{}", name);
            self.world.rooms.insert(
                id.clone(),
                Room {
                    name: room.name,
                    exits: room.exits,
                    description: room.description,
                    npc: room.npc,
                    items: room.items.into_iter().map(OwnedItem::from).collect(),
                },
            );
            self.define(&id, &path)?;
        }

        for (name, item) in parsed.item {
            let id = format!("item.{}", name);
            self.world.items.insert(id.clone(), item);
            self.define(&id, &path)?;
        }

        for (name, quest) in parsed.quest {
            let id = format!("quest.{}", name);
            self.world.quests.insert(id.clone(), quest);
            self.define(&id, &path)?;
        }
        Ok(())
    }

    fn check_refs(&self) -> Result<(), ConfigError> {
        for (id, obj) in self.referencing_entries() {
            for r in obj {
                self.definer.get(r).ok_or(ConfigError::DanglingRef {
                    from_id: id.to_string(),
                    missing_ref: r.to_string(),
                })?;
            }
        }
        Ok(())
    }

    fn get_visible_file(&self, start: &Path) -> HashSet<PathBuf> {
        let mut visibles: HashSet<PathBuf> = HashSet::new();
        let mut queue = vec![start.to_path_buf()];

        while let Some(f) = queue.pop() {
            
            if visibles.contains(&f) {
                continue;
            }
            visibles.insert(f.clone());
            for import in self.import_graph.get(&f).unwrap_or(&Vec::new()) {
                queue.push(import.to_path_buf());
            }
        }
        visibles
    }

    fn check_scope(&self) -> Result<(), ConfigError> {
        for (id, obj) in self.referencing_entries() {
            let f = &self.definer[id];
            let visibles = self.get_visible_file(f);

            for r in obj {
                let g = &self.definer[r];

                if !visibles.contains(g) {
                    return Err(ConfigError::ScopeViolation {
                        from_id: id.to_string(),
                        ref_id: r.to_string(),
                        defined_in: g.to_path_buf(),
                    });
                }
            }
        }

        Ok(())
    }

    fn check_singles_type(&self) -> Result<(), ConfigError> {
        for (id, r) in self.singletons() {
            if !self.world.rooms.contains_key(r) {
                return Err(ConfigError::WrongRef {
                    from_id: id.to_string(),
                    ref_id: r.to_string(),
                });
            }
        }
        Ok(())
    }

    fn finish(self) -> World {
        self.world
    }
}

pub(super) fn load(entry: &Path) -> Result<World, ConfigError> {
    let mut loader = Loader::new();
    loader.load_file(entry)?;
    if loader.world.spawn_room.is_empty() {
        return Err(ConfigError::MissingSpawnPoint);
    }
    if loader.world.gambling_room.is_empty() {
        return Err(ConfigError::MissingGamblingRoom);
    }
    if loader.world.dungeon_entrance.is_empty() {
        return Err(ConfigError::MissingDungeonEntrance);
    }
    loader.check_refs()?;
    loader.check_scope()?;
    loader.check_singles_type()?;
    Ok(loader.finish())
}
