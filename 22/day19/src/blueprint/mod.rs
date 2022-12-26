mod resource;
mod resources;

use std::collections::HashMap;

pub use resources::Resources;

pub use resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u8,
    pub bots: HashMap<Resource, Resources>,
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let (number, costs) = value.split_once(":").unwrap();

        let (_, number) = number.split_once(" ").unwrap();
        let id = number.parse::<u8>().unwrap();
        let mut bots = HashMap::new();
        for res in costs.split(".") {
            if res.is_empty() {
                continue;
            }
            let res = res.strip_prefix(" Each ").unwrap();

            let (res, costs) = res.split_once(" robot costs ").unwrap();

            let res = Resource::from(res);
            let cost = Resources::from(costs);

            bots.insert(res, cost);
        }
        Blueprint { id, bots }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_works() {
        let blueprint : Blueprint = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".into();

        assert_eq!(blueprint.id, 1);
        assert_eq!(blueprint.bots[&Resource::Ore], Resources::new(4, 0, 0, 0));
        assert_eq!(blueprint.bots[&Resource::Clay], Resources::new(2, 0, 0, 0));
        assert_eq!(
            blueprint.bots[&Resource::Obsidian],
            Resources::new(3, 14, 0, 0)
        );
        assert_eq!(blueprint.bots[&Resource::Geode], Resources::new(2, 0, 7, 0));
    }
}
