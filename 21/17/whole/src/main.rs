use std::cmp::Ordering;

struct CoordRange {
    min: i32,
    max: i32,
}

impl CoordRange {
    fn in_range(&self, value: i32) -> std::cmp::Ordering {
        if value < self.min {
            Ordering::Less
        } else if value > self.max {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
    fn zero() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct TargetRange {
    x: CoordRange,
    y: CoordRange,
}

enum FireResult {
    Hit(Vec<Vec2>),
    Miss,
}

enum PositionResult {
    Possible,
    InRange,
    Impossible,
}

impl TargetRange {
    fn check_position(&self, pos: Vec2) -> PositionResult {
        let x_result = self.x.in_range(pos.x);
        let y_result = self.y.in_range(pos.y);

        if x_result == Ordering::Greater || y_result == Ordering::Less {
            PositionResult::Impossible
        } else if x_result == Ordering::Equal && y_result == Ordering::Equal {
            PositionResult::InRange
        } else {
            PositionResult::Possible
        }
    }

    fn fire(&self, mut probe: &mut Probe) -> FireResult {
        let mut positions = vec![probe.pos];
        loop {
            (*probe).step();

            let position_result = self.check_position(probe.pos);

            match position_result {
                PositionResult::Impossible => { return FireResult::Miss; }
                PositionResult::Possible => { positions.push(probe.pos) }
                PositionResult::InRange => {
                    positions.push(probe.pos);
                    return FireResult::Hit(positions);
                }
            }
        }
    }
}

struct Probe {
    vel: Vec2,
    pos: Vec2,
}

impl Probe {
    fn new(vel: Vec2) -> Self {
        Probe {
            vel,
            pos: Vec2::zero(),
        }
    }

    fn step(&mut self) {
        self.pos += self.vel;

        let vel_delta = match self.vel {
            Vec2 { x, y: _ } if x > 0 => Vec2::new(-1, -1),
            Vec2 { x, y: _ } if x < 0 => Vec2::new(1, -1),
            _ => Vec2::new(0, -1)
        };

        self.vel += vel_delta;
    }
}

fn main() {
    let target = TargetRange {
        x: CoordRange { min: 137, max: 171 },
        y: CoordRange { min: -98, max: -73 },
    };

    let mut min_x = 0;

    loop {
        min_x += 1;
        let sum = min_x * (min_x + 1) / 2;

        if sum >= target.x.min {
            break;
        }
    }

    let max_x = target.x.max + 1;
    println!("{min_x} {max_x}");

    let mut total_max_y = i32::MIN;
    let mut count = 0u128;
    for init_y in (target.y.min - 1).. {
        let mut max_y = i32::MIN;
        for init_x in min_x..=max_x {
            let mut probe = Probe::new(Vec2::new(init_x, init_y));

            if let FireResult::Hit(positions) = target.fire(&mut probe) {
                max_y = max_y.max(positions.iter().map(|p| p.y).max().unwrap());
                count += 1;
            }
        }

        if max_y > total_max_y {
            total_max_y = max_y;
        }

        println!("{init_y} {total_max_y} {count}");
    }

    println!("{total_max_y}")

    // target area: x=137..171, y=-98..-73"
}

#[cfg(test)]
mod tests {
    use crate::{CoordRange, FireResult, Probe, TargetRange, Vec2};

    #[test]
    fn probe_moves() {
        let mut probe = Probe::new(Vec2::new(5, 5));

        probe.step();

        assert_eq!(probe.pos, Vec2::new(5, 5));
    }

    #[test]
    fn probe_reaches_zero_x_velocity() {
        let mut probe = Probe::new(Vec2::new(1, 5));

        probe.step();
        probe.step();
        probe.step();

        assert_eq!(probe.vel.x, 0);
    }

    #[test]
    fn probe_reaches_zero_x_velocity_from_negative() {
        let mut probe = Probe::new(Vec2::new(-2, 5));

        probe.step();
        probe.step();
        probe.step();

        assert_eq!(probe.vel.x, 0);
    }


    #[test]
    fn probe_decreases_y_velocity_per_step() {
        let mut probe = Probe::new(Vec2::new(0, 1));

        probe.step();
        probe.step();
        probe.step();

        assert_eq!(probe.vel.y, -2);
    }

    #[test]
    fn probe_moves_correctly_after_multiple_steps() {
        let mut probe = Probe::new(Vec2::new(5, 2));

        probe.step();
        probe.step();
        probe.step();

        assert_eq!(probe.pos, Vec2::new(12, 3));
    }

    fn verify_fire_result(target_range: &TargetRange, probe: &mut Probe, final_position: Vec2) {
        let fire_result = target_range.fire(probe);

        if let FireResult::Hit(mut positions) = fire_result {
            assert_eq!(positions.pop(), Some(final_position))
        } else {
            panic!();
        }
    }

    #[test]
    fn probe_hits_target_range() {
        let target_range = TargetRange {
            x: CoordRange { min: 20, max: 30 },
            y: CoordRange { min: -10, max: -5 },
        };
        let mut probe = Probe::new(Vec2::new(7, 2));

        verify_fire_result(&target_range, &mut probe, Vec2::new(28, -7));

        let mut probe = Probe::new(Vec2::new(6, 3));
        verify_fire_result(&target_range, &mut probe, Vec2::new(21, -9));
    }

    #[test]
    fn probe_misses_if_wrong_velocity() {
        let target_range = TargetRange {
            x: CoordRange { min: 20, max: 30 },
            y: CoordRange { min: -10, max: -5 },
        };

        let mut probe = Probe::new(Vec2::new(17, -4));

        match target_range.fire(&mut probe) {
            FireResult::Hit(_) => { panic!("Should not hit!") }
            FireResult::Miss => {}
        }
    }
}