mod structures;

use serde_yaml;
use crate::structures::items::{self, Items};
use crate::structures::enums::item_kind::ItemKind;

fn main() {
    let yaml_data = "
name: Bouclier
price: 300
number: 1
kind:
 Weapon:
  damages: 15
";

    let deserialized_config: Items = serde_yaml::from_str(yaml_data).unwrap();
    println!("{:?}", deserialized_config);
}
