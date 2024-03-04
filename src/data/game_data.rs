use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::data::entity::DataEntity;
use crate::data::names_pack::NamesPack;
use crate::data::ship_class::ShipClass;

fn make_str(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}

#[derive(Debug)]
pub struct GameData {
    pub ships: HashMap<&'static str, ShipClass>,
    pub names: Vec<NamesPack>,
}

impl GameData {
    pub fn load() -> GameData {
        let mut data = GameData {
            ships: HashMap::with_capacity(1),
            names: Vec::with_capacity(3),
        };
        let path: PathBuf = ["data", "core"].iter().collect();
        data.load_dir(&path);
        data
    }

    fn load_dir(&mut self, path: &Path) {
        for entry in path
            .read_dir()
            .unwrap_or_else(|e| panic!("Could not read dir {path:?}: {e:?}"))
        {
            let path = entry.unwrap().path();
            if path.is_dir() {
                self.load_dir(&path);
            } else if let Ok(file) = File::open(path) {
                if let Ok(entities) =
                    serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file))
                {
                    for entity in entities {
                        self.add_entity(entity);
                    }
                }
            }
        }
    }

    fn add_entity(&mut self, entity: DataEntity) {
        match entity {
            DataEntity::ShipClass(ship) => {
                self.ships.insert(make_str(ship.id.as_str()), ship);
            }
            DataEntity::NamesPack(name_pack) => self.names.push(name_pack),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game_data::GameData;

    #[test]
    fn test_load() {
        let data = GameData::load();
        let dugong = data.ships.get("dugong").unwrap();
        assert_eq!(dugong.name, "Dugong");
        assert_eq!(dugong.tiles.len() as i32, dugong.bounds.0 * dugong.bounds.1);
        assert_eq!(dugong.tiles.as_slice()[30], "@");
        assert!(!data.names.is_empty());
    }
}
