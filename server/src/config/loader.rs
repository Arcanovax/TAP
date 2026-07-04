use crate::{
    config::ConfigError,
    structures::{
        enums::exits::Direction,
        game::World,
        item::Item,
        npc::NPC,
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
    npc: HashMap<String, NPC>,
    #[serde(default)]
    item: HashMap<String, Item>,
    #[serde(default)]
    room: HashMap<String, ConfigRoom>,
    #[serde(default)]
    quest: HashMap<String, Quest>,
    #[serde(default)]
    spawn_point: String,
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
        if !parsed.spawn_point.is_empty() {
            if let Some(file_a) = self.definer.insert(SPAWN_POINT.to_string(), path.clone()) {
                return Err(ConfigError::Conflict {
                    id: SPAWN_POINT.to_string(),
                    file_a,
                    file_b: path,
                });
            }
            self.world.spawn_room = parsed.spawn_point;
        }
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
        for (name, npc) in parsed.npc {
            let id = format!("npc.{}", name);
            if let Some(file_a) = self.definer.insert(id.clone(), path.clone()) {
                return Err(ConfigError::Conflict {
                    id,
                    file_a,
                    file_b: path,
                });
            };
            for (d, _) in &npc.dialog {
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
            if let Some(file_a) = self.definer.insert(id.clone(), path.clone()) {
                return Err(ConfigError::Conflict {
                    id,
                    file_a,
                    file_b: path,
                });
            };
        }
        for (name, item) in parsed.item {
            let id = format!("item.{}", name);
            self.world.items.insert(id.clone(), item);
            if let Some(file_a) = self.definer.insert(id.clone(), path.clone()) {
                return Err(ConfigError::Conflict {
                    id,
                    file_a,
                    file_b: path,
                });
            };
        }
        for (name, quest) in parsed.quest {
            let id = format!("quest.{}", name);
            self.world.quests.insert(id.clone(), quest);
            if let Some(file_a) = self.definer.insert(id.clone(), path.clone()) {
                return Err(ConfigError::Conflict {
                    id,
                    file_a,
                    file_b: path,
                });
            };
        }
        Ok(())
    }

    fn check_refs(&self) -> Result<(), ConfigError> {
        for (id, room) in &self.world.rooms {
            for r in room.references() {
                self.definer.get(r).ok_or(ConfigError::DanglingRef {
                    from_id: id.clone(),
                    missing_ref: r.to_string(),
                })?;
            }
        }
        for (id, npc) in &self.world.npcs {
            for r in npc.references() {
                self.definer.get(r).ok_or(ConfigError::DanglingRef {
                    from_id: id.clone(),
                    missing_ref: r.to_string(),
                })?;
            }
        }
        for (id, quest) in &self.world.quests {
            for r in quest.references() {
                self.definer.get(r).ok_or(ConfigError::DanglingRef {
                    from_id: id.clone(),
                    missing_ref: r.to_string(),
                })?;
            }
        }

        self.definer
            .get(&self.world.spawn_room)
            .ok_or(ConfigError::DanglingRef {
                from_id: SPAWN_POINT.to_string(),
                missing_ref: self.world.spawn_room.clone(),
            })?;

        Ok(())
    }

    fn get_visible_file(&self, start: &Path) -> HashSet<PathBuf> {
        let mut visibles: HashSet<PathBuf> = HashSet::new();
        let mut queue = vec![start.to_path_buf()];

        while queue.len() != 0 {
            let f = queue.pop().unwrap();
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
        for (id, room) in &self.world.rooms {
            let f = &self.definer[id];
            let visibles = self.get_visible_file(&f);

            for r in room.references() {
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
        for (id, npc) in &self.world.npcs {
            let f = &self.definer[id];
            let visibles = self.get_visible_file(&f);

            for r in npc.references() {
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
        for (id, quest) in &self.world.quests {
            let f = &self.definer[id];
            let visibles = self.get_visible_file(&f);

            for r in quest.references() {
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

        let f = &self.definer[SPAWN_POINT];
        let visibles = self.get_visible_file(&f);
        let g = &self.definer[&self.world.spawn_room];
        if !visibles.contains(g) {
            return Err(ConfigError::ScopeViolation {
                from_id: SPAWN_POINT.to_string(),
                ref_id: self.world.spawn_room.clone(),
                defined_in: g.to_path_buf(),
            });
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
    loader.check_refs()?;
    loader.check_scope()?;
    Ok(loader.finish())
}
