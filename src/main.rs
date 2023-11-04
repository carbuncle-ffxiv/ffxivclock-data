use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub slot: Value,
    pub name: String,
    pub name_fr: String,
    pub name_de: String,
    pub level: String,
    pub gathering_skill: String,
    pub perception: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub description_fr: Option<String>,
    pub description_de: Option<String>,
    pub patch: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: Option<String>,
    pub zone: String,
    pub zone_fr: String,
    pub zone_de: String,
    pub teleport: String,
    pub teleport_fr: String,
    pub teleport_de: String,
    pub position: Value,
    pub start_time: String,
    pub end_time: String,
    pub item_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LocalizedString {
    en: String,
    fr: String,
    de: String,
}

impl LocalizedString {
    fn new(en: String, fr: String, de: String) -> Self {
        Self { en, fr, de }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct OutputItem {
    id: String,
    slot: i8,
    name: LocalizedString,
    level: String,
    gathering_skill: String,
    perception: String,
    image_url: String,
    description: LocalizedString,
}

#[derive(Debug, Deserialize, Serialize)]
struct OutputNode {
    id: String,
    #[serde(rename = "type")]
    _type: String,
    sub_type: String,
    zone: LocalizedString,
    teleport: LocalizedString,
    position: Position,
    start_time: String,
    end_time: String,
    items: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Position {
    x: i64,
    y: i64,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let items = include_str!("../items.json");
    let items: Items = serde_json::from_str(items).unwrap();
    let nodes = include_str!("../nodes.json");
    let nodes: Nodes = serde_json::from_str(nodes).unwrap();

    let mut output_items = Vec::new();

    for item in items.items {
        let slot = match item.slot {
            Value::Number(slot) => slot.as_i64().unwrap() as i8,
            _ => 0,
        };
        let image_url = match item.image_url {
            Some(image_url) => image_url,
            None => String::new(),
        };
        let output_item = OutputItem {
            id: item.id,
            slot,
            name: LocalizedString::new(item.name, item.name_fr, item.name_de),
            level: item.level,
            gathering_skill: item.gathering_skill,
            perception: item.perception,
            image_url,
            description: LocalizedString::new(
                item.description.unwrap_or_default(),
                item.description_fr.unwrap_or_default(),
                item.description_de.unwrap_or_default(),
            ),
        };
        output_items.push(output_item);
    }

    let output_items = serde_json::to_string_pretty(&output_items).unwrap();
    std::fs::write("items.out.json", output_items).unwrap();

    let mut output_nodes = Vec::new();

    for node in nodes.nodes {
        println!("{:?}", node);

        let position = match node.position {
            Value::Object(position) => {
                if let (Some(x), Some(y)) = (position.get("x"), position.get("y")) {
                    if let (Some(x), Some(y)) = (x.as_i64(), y.as_i64()) {
                        Position { x, y }
                    } else {
                        Position::default()
                    }
                } else {
                    Position::default()
                }
            }
            _ => Position::default(),
        };

        let sub_type = match node.sub_type {
            Some(sub_type) => sub_type,
            None => String::new(),
        };

        let output_node = OutputNode {
            id: node.id,
            _type: node.type_field,
            sub_type,
            zone: LocalizedString::new(node.zone, node.zone_fr, node.zone_de),
            teleport: LocalizedString::new(node.teleport, node.teleport_fr, node.teleport_de),
            position,
            start_time: node.start_time,
            end_time: node.end_time,
            items: node.item_ids,
        };

        output_nodes.push(output_node);
    }

    let output_nodes = serde_json::to_string_pretty(&output_nodes).unwrap();

    std::fs::write("nodes.out.json", output_nodes).unwrap();
}
