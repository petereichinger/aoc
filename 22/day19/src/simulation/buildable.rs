use crate::blueprint::Resources;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Buildable {
    Now,
    In(u8),
    Never,
}

pub(super) fn can_build(
    current_resources: &Resources,
    population: &Resources,
    bot_costs: &Resources,
) -> Buildable {
    let all_fulfilled = current_resources
        .iter()
        .all(|(r, amount)| amount >= &bot_costs[r]);

    if all_fulfilled {
        return Buildable::Now;
    }

    let turns: Vec<_> = bot_costs
        .iter()
        .filter(|(_, &amount)| amount > 0)
        .map(|(r, amount)| {
            if population[r] == 0 {
                None
            } else {
                let remaining_resource = amount.saturating_sub(current_resources[r]);
                let turns = remaining_resource / population[r];
                let remainder = remaining_resource % population[r];
                Some(if remainder == 0 { turns } else { turns + 1 })
            }
        })
        .collect();

    if turns.iter().any(|e| e.is_none()) {
        return Buildable::Never;
    }

    let max_turns = turns
        .iter()
        .map(|e| e.unwrap_or(0))
        .max()
        .expect("no None items should be present");

    Buildable::In(max_turns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildable_now() {
        let can_build = can_build(
            &Resources::new(4, 0, 0, 0),
            &Resources::new(1, 0, 0, 0),
            &Resources::new(2, 0, 0, 0),
        );

        assert_eq!(can_build, Buildable::Now);
    }

    #[test]
    fn test_buildable_never() {
        let can_build = can_build(
            &Resources::new(4, 0, 0, 0),
            &Resources::new(1, 0, 0, 0),
            &Resources::new(0, 1, 0, 0),
        );

        assert_eq!(can_build, Buildable::Never);
    }

    #[test]
    fn test_buildable_in_a_few_turns() {
        let can_build = can_build(
            &Resources::new(0, 0, 0, 0),
            &Resources::new(1, 0, 0, 0),
            &Resources::new(4, 0, 0, 0),
        );

        assert_eq!(can_build, Buildable::In(4));
    }

    #[test]
    fn test_buildable_in_ceils_correctly() {
        let buildable = can_build(
            &Resources::new(0, 0, 0, 0),
            &Resources::new(2, 0, 0, 0),
            &Resources::new(5, 0, 0, 0),
        );

        assert_eq!(buildable, Buildable::In(3));

        let buildable = can_build(
            &Resources::new(0, 0, 0, 0),
            &Resources::new(3, 0, 0, 0),
            &Resources::new(5, 0, 0, 0),
        );

        assert_eq!(buildable, Buildable::In(2));
    }

    #[test]
    fn test_buildable_accounts_for_current_resource() {
        let buildable = can_build(
            &Resources::new(2, 0, 0, 0),
            &Resources::new(1, 0, 0, 0),
            &Resources::new(5, 0, 0, 0),
        );

        assert_eq!(buildable, Buildable::In(3));

        let buildable = can_build(
            &Resources::new(2, 0, 0, 0),
            &Resources::new(2, 0, 0, 0),
            &Resources::new(5, 0, 0, 0),
        );

        assert_eq!(buildable, Buildable::In(2));
    }
}
