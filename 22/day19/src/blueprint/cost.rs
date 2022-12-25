use super::resource::Resource;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cost {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
}

impl From<&str> for Cost {
    fn from(value: &str) -> Self {
        let mut cost: Cost = Default::default();
        for res in value.split(" and ") {
            let (amount, r_type) = res.split_once(" ").unwrap();
            let amount = amount.parse::<u8>().unwrap();

            let r_type = Resource::from(r_type);

            match r_type {
                Resource::Ore => cost.ore = amount,
                Resource::Clay => cost.clay = amount,
                Resource::Obsidian => cost.obsidian = amount,
                _ => {}
            }
        }

        cost
    }
}
