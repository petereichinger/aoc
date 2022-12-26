mod buildable;

use std::collections::HashMap;

use crate::blueprint::{Blueprint, Resource, Resources};

use self::buildable::{can_build, Buildable};

#[derive(Debug, Clone)]
pub struct Simulation {
    pub blueprint: Blueprint,
    max_population: Resources,
}

impl Simulation {
    pub fn for_blueprint(blueprint: Blueprint) -> Self {
        let mut max_population = blueprint
            .bots
            .iter()
            .map(|(r, c)| c)
            .fold(Resources::default(), |a, e| a.max(e));

        max_population[Resource::Geode] = u8::MAX;
        Self {
            blueprint,
            max_population,
        }
    }

    fn simulation_step(
        &self,
        resources: Resources,
        population: Resources,
        remaining_time: u8,
    ) -> u8 {
        if remaining_time == 0 {
            return resources[Resource::Geode];
        }
        let mut max_geodes = resources[Resource::Geode];
        for (bot_type, bot_costs) in &self.blueprint.bots {
            let pop_limit = self.max_population[bot_type];
            if pop_limit == population[bot_type] {
                continue;
            }

            let buildable = can_build(&resources, &population, bot_costs);
            let result = match buildable {
                Buildable::Never => continue,
                Buildable::Now => {
                    // decrease resources
                    let mut new_resources = resources.clone();
                    new_resources -= bot_costs;
                    // get resources
                    new_resources += &population;
                    // add pop
                    let mut new_population = population.clone();
                    new_population[bot_type] += 1;
                    // recurse with one time less
                    self.simulation_step(new_resources, new_population, remaining_time - 1)
                }
                Buildable::In(x) => {
                    if x + 1 >= remaining_time {
                        continue;
                    }
                    // increase resources x times
                    let mut new_resources = resources.clone();
                    for _ in 0..=x {
                        new_resources += &population;
                    }

                    //decrease resources
                    new_resources -= bot_costs;

                    // add population
                    let mut new_population = population.clone();
                    new_population[bot_type] += 1;

                    // recurse with x + 1 less minutes

                    self.simulation_step(new_resources, new_population, remaining_time - (x + 1))
                }
            };
            max_geodes = max_geodes.max(result);
        }

        max_geodes
    }

    pub fn simulate(&self) -> u8 {
        let resources = Resources::default();
        let population = Resources::new(1, 0, 0, 0);

        let remaining_time = 24;

        self.simulation_step(resources, population, remaining_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLUEPRINT_1 :&str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

    #[test]
    fn test_simulation() {
        let simulation = Simulation::for_blueprint(BLUEPRINT_1.into());

        let _result = simulation.simulate();
    }

    #[test]
    fn test_max_pop() {
        let simulation = Simulation::for_blueprint(BLUEPRINT_1.into());

        assert_eq!(simulation.max_population[Resource::Ore], 4);
        assert_eq!(simulation.max_population[Resource::Clay], 14);
        assert_eq!(simulation.max_population[Resource::Obsidian], 7);
        assert_eq!(simulation.max_population[Resource::Geode], u8::MAX);
    }
}
