#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl From<&str> for Resource {
    fn from(value: &str) -> Self {
        match value.trim() {
            "ore" => Resource::Ore,
            "clay" => Resource::Clay,
            "obsidian" => Resource::Obsidian,
            "geode" => Resource::Geode,
            x => panic!("Invalid resource {x}"),
        }
    }
}
