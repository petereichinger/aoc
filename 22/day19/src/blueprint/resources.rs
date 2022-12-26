use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign},
};

use super::resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resources {
    entries: HashMap<Resource, u8>,
}

impl Resources {
    pub fn new(ore: u8, clay: u8, obsidian: u8, geode: u8) -> Self {
        Self {
            entries: [
                (Resource::Ore, ore),
                (Resource::Clay, clay),
                (Resource::Obsidian, obsidian),
                (Resource::Geode, geode),
            ]
            .into(),
        }
    }

    pub fn max(mut self, other: &Resources) -> Self {
        self.entries.iter_mut().for_each(|(r, c)| {
            let other_amount = other.entries[r];
            *c = other_amount.max(*c);
        });

        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Resource, &u8)> {
        RESOURCE_ORDER
            .iter()
            .map(|r| (r, self.entries.get(r).expect("invalid resource found")))
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            entries: DEFAULT_COSTS.into(),
        }
    }
}

impl From<&str> for Resources {
    fn from(value: &str) -> Self {
        let mut cost: Resources = Default::default();
        for res in value.split(" and ") {
            let (amount, r_type) = res.split_once(" ").unwrap();
            let amount = amount.parse::<u8>().unwrap();

            let r_type = Resource::from(r_type);

            cost.entries.insert(r_type, amount);
        }

        cost
    }
}

impl Add for &Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            entries: self
                .iter()
                .zip(rhs.iter())
                .map(|(s, rhs)| (*s.0, s.1 + rhs.1))
                .collect(),
        }
    }
}

impl Sub for &Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            entries: self
                .iter()
                .zip(rhs.iter())
                .map(|(s, rhs)| (*s.0, s.1.saturating_sub(*rhs.1)))
                .collect(),
        }
    }
}

impl AddAssign<&Resources> for Resources {
    fn add_assign(&mut self, rhs: &Resources) {
        for resource in &RESOURCE_ORDER {
            let target = self.entries.get_mut(resource).unwrap();
            let rhs = rhs.entries.get(resource).unwrap();

            *target += rhs;
        }
    }
}

impl SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, rhs: &Resources) {
        for resource in &RESOURCE_ORDER {
            let target = self.entries.get_mut(resource).unwrap();
            let rhs = rhs.entries.get(resource).unwrap();

            *target -= rhs;
        }
    }
}

const RESOURCE_ORDER: [Resource; 4] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

const DEFAULT_COSTS: [(Resource, u8); 4] = [
    (Resource::Ore, 0),
    (Resource::Clay, 0),
    (Resource::Obsidian, 0),
    (Resource::Geode, 0),
];

impl Index<Resource> for Resources {
    type Output = u8;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.entries[&index]
    }
}

impl IndexMut<Resource> for Resources {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        self.entries.get_mut(&index).unwrap()
    }
}

impl Index<&Resource> for Resources {
    type Output = u8;

    fn index(&self, index: &Resource) -> &Self::Output {
        self.entries.get(index).expect("got unexpected resource")
    }
}

impl IndexMut<&Resource> for Resources {
    fn index_mut(&mut self, index: &Resource) -> &mut Self::Output {
        self.entries
            .get_mut(index)
            .expect("got unexpected resource")
    }
}

#[cfg(test)]
mod tests {
    use crate::blueprint::Resource;

    use super::Resources;

    #[test]
    fn test_maximum() {
        let cost_a = Resources {
            entries: [
                (Resource::Ore, 1),
                (Resource::Clay, 2),
                (Resource::Obsidian, 0),
            ]
            .into(),
        };

        let cost_b = Resources {
            entries: [
                (Resource::Ore, 0),
                (Resource::Clay, 4),
                (Resource::Obsidian, 3),
            ]
            .into(),
        };

        let max = cost_a.max(&cost_b);

        assert_eq!(max.entries[&Resource::Ore], 1);
        assert_eq!(max.entries[&Resource::Clay], 4);
        assert_eq!(max.entries[&Resource::Obsidian], 3);
    }
}
