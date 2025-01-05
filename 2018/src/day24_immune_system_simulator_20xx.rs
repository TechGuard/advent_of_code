use regex::Regex;

pub static DAY: u32 = 24;
pub static EXAMPLE_INPUT: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

const DEBUG: bool = false;

#[derive(Debug)]
struct Army<'a> {
    #[allow(unused)]
    name: &'a str,
    groups: Vec<Group<'a>>,
}

#[derive(Debug, Clone, Copy)]
struct GroupIndex(usize, usize);

#[derive(Debug)]
struct Group<'a> {
    index: GroupIndex,
    unit_count: usize,
    unit_health: usize,
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
    attack_damage: usize,
    attack_type: &'a str,
    initiative: usize,
}

struct TargetSelection {
    attacker: GroupIndex,
    defender: GroupIndex,
}

impl GroupIndex {
    fn get<'a, 'b>(&self, armies: &'b Vec<Army<'a>>) -> &'b Group<'a> {
        &armies[self.0].groups[self.1]
    }
    fn get_mut<'a, 'b>(&self, armies: &'b mut Vec<Army<'a>>) -> &'b mut Group<'a> {
        &mut armies[self.0].groups[self.1]
    }
}

impl<'a> Group<'a> {
    fn effective_power(&self) -> usize {
        self.unit_count * self.attack_damage
    }

    fn calc_damage(&self, other: &Self) -> usize {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

impl<'a> Army<'a> {
    fn target_selection(&self, other: &Self, targets: &mut Vec<TargetSelection>) {
        let mut attackers: Vec<_> = self.groups.iter().filter(|g| g.unit_count > 0).collect();
        let mut defenders: Vec<_> = other.groups.iter().filter(|g| g.unit_count > 0).collect();

        // Sort attackers by effective power and initiative
        attackers.sort_unstable_by(|a, b| {
            b.effective_power()
                .cmp(&a.effective_power())
                .then_with(|| b.initiative.cmp(&a.initiative))
        });

        for attacker in attackers {
            // Sort defenders by damage, effective power and initiative
            defenders.sort_by(|a, b| {
                attacker
                    .calc_damage(a)
                    .cmp(&attacker.calc_damage(b))
                    .then_with(|| {
                        a.effective_power()
                            .cmp(&b.effective_power())
                            .then_with(|| a.initiative.cmp(&b.initiative))
                    })
            });

            // Pick target
            if let Some(&defender) = defenders.last() {
                let damage = attacker.calc_damage(defender);
                if damage > 0 {
                    defenders.pop();
                    targets.push(TargetSelection {
                        attacker: attacker.index,
                        defender: defender.index,
                    });
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Option<Vec<Army>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)(\d+) units each with (\d+) hit points( \(([^)]+)\))? with an attack that does (\d+) ([^ ]+) damage at initiative (\d+)").unwrap();
    }
    let mut armies = Vec::with_capacity(2);
    for input in input.split("\n\n") {
        let mut groups = Vec::new();
        for group in RE.captures_iter(input) {
            let mut weaknesses = Vec::new();
            let mut immunities = Vec::new();
            if let Some(m) = group.get(4) {
                for desc in m.as_str().split("; ") {
                    if let Some(desc) = desc.strip_prefix("weak to ") {
                        for weakness in desc.split(", ") {
                            weaknesses.push(weakness);
                        }
                    } else if let Some(desc) = desc.strip_prefix("immune to ") {
                        for immunity in desc.split(", ") {
                            immunities.push(immunity);
                        }
                    } else {
                        unreachable!();
                    }
                }
            }
            groups.push(Group {
                index: GroupIndex(armies.len(), groups.len()),
                unit_count: group.get(1)?.as_str().parse().ok()?,
                unit_health: group.get(2)?.as_str().parse().ok()?,
                weaknesses,
                immunities,
                attack_damage: group.get(5)?.as_str().parse().ok()?,
                attack_type: group.get(6)?.as_str(),
                initiative: group.get(7)?.as_str().parse().ok()?,
            });
        }
        armies.push(Army {
            name: input.lines().next()?.strip_suffix(':')?,
            groups,
        });
    }
    Some(armies)
}

pub fn main(input: &str) -> (usize, String) {
    let mut armies = parse_input(input).unwrap();

    let ans1 = loop {
        if DEBUG {
            for army in armies.iter() {
                println!("{}:", army.name);
                for group in army.groups.iter().filter(|g| g.unit_count > 0) {
                    println!(
                        "Group {} contains {} units",
                        group.index.1 + 1,
                        group.unit_count
                    );
                }
            }
            println!();
        }

        // Selection phase
        let mut targets = Vec::new();
        for i in 0..armies.len() {
            armies[i].target_selection(&armies[(i + 1) % armies.len()], &mut targets);
        }

        // Attacking phase, sort by initiative
        targets.sort_by(|a, b| {
            a.attacker
                .get(&armies)
                .initiative
                .cmp(&b.attacker.get(&armies).initiative)
        });
        for TargetSelection { attacker, defender } in targets.iter().rev() {
            // Attack the defending unit
            let damage = attacker.get(&armies).calc_damage(defender.get(&armies));
            let units_killed = {
                let defender = defender.get_mut(&mut armies);
                let units_to_kill = damage / defender.unit_health;
                let units_before = defender.unit_count;
                if units_to_kill < defender.unit_count {
                    defender.unit_count -= units_to_kill;
                } else {
                    defender.unit_count = 0;
                }
                units_before - defender.unit_count
            };

            if DEBUG {
                println!(
                    "{} group {} attacks defending group {}, killing {} units",
                    armies[attacker.0].name,
                    attacker.1 + 1,
                    defender.1 + 1,
                    units_killed
                );
            }
        }

        if DEBUG {
            println!();
        }

        // Count units for all armies
        let count: Vec<usize> = armies
            .iter()
            .map(|a| a.groups.iter().map(|g| g.unit_count).sum())
            .collect();

        // End battle if one army has no units
        if count.contains(&0) {
            break count.iter().sum();
        }
    };

    (ans1, String::from("Not implemented"))
}
