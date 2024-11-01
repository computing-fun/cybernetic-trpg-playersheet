use serde::{Deserialize, Serialize};

use crate::book::Sheet;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default, transparent)]
pub struct Ability {
    score: isize,
}

impl Ability {
    pub const MIN_SCORE: isize = 1;
    pub const MAX_SCORE: isize = 30;

    /// Returns the ability score.
    pub fn score(&self) -> isize {
        self.score
    }

    /// Calculates the modifier for the score.
    /// Modifier is score minus 10 divided by 2, rounded down.
    pub fn modifier(&self) -> isize {
        self.score.saturating_sub(10).div_euclid(2)
    }

    /// Creates an `Ability` from a given score.
    /// The score will be clamped between `MIN_SCORE` and `MAX_SCORE`.
    pub fn from_score(score: isize) -> Self {
        Self {
            score: score.clamp(Ability::MIN_SCORE, Ability::MAX_SCORE),
        }
    }

    /// Calculates the lowest score that would produce the specified modifier.
    pub fn from_modifier_down(modifier: isize) -> Self {
        Self::from_score(modifier.saturating_mul(2).saturating_add(10))
    }

    /// Calculates the highest score that would produce the specified modifier.
    pub fn from_modifier_up(modifier: isize) -> Self {
        Self::from_score(modifier.saturating_mul(2).saturating_add(11))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AbilityBlock {
    /// Strength measures bodily power, athletic training, and the extent to which you can exert raw physical force.
    pub strenght: Ability,

    /// Dexterity measures agility, reflexes, and balance.
    pub dexterity: Ability,

    /// Constitution measures health, stamina, and vital force.
    pub constitution: Ability,

    /// Intelligence measures mental acuity, accuracy of recall, and the ability to reason.
    pub intellignce: Ability,

    /// Wisdom reflects how attuned you are to the world around you and represents perceptiveness and intuition.
    pub wisdom: Ability,

    /// Charisma measures your ability to interact effectively with others. It includes such factors as confidence and eloquence, and it can represent a charming or commanding personality.
    pub charisma: Ability,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub speed: usize,
    pub race: Race,
    #[serde(flatten)]
    pub ability: AbilityBlock,
    pub class: Vec<Class>,
}

impl Sheet for Character {
    const TYPE: &'static str = "Character";
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Race {
    pub name: String,
}

impl Sheet for Race {
    const TYPE: &'static str = "Race";
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Class {
    pub name: String,
    pub level: usize,
}

impl Sheet for Class {
    const TYPE: &'static str = "Class";
    fn name(&self) -> String {
        self.name.clone()
    }
}

mod meta {
    use std::ops::{Add, Sub};

    use super::*;

    // - Ability - AbilityBlock -

    impl Default for Ability {
        fn default() -> Self {
            Self::from_score(10)
        }
    }

    impl Add for Ability {
        type Output = Ability;
        fn add(self, rhs: Self) -> Self::Output {
            Ability::from_score(self.score.saturating_add(rhs.score))
        }
    }

    impl Sub for Ability {
        type Output = Ability;
        fn sub(self, rhs: Self) -> Self::Output {
            Ability::from_score(self.score.saturating_sub(rhs.score))
        }
    }

    impl Default for AbilityBlock {
        fn default() -> Self {
            Self {
                strenght: Default::default(),
                dexterity: Default::default(),
                constitution: Default::default(),
                intellignce: Default::default(),
                wisdom: Default::default(),
                charisma: Default::default(),
            }
        }
    }

    // - Character -

    impl Default for Character {
        fn default() -> Self {
            Self {
                name: "Unknown".to_string(),
                race: Default::default(),
                class: Default::default(),
                description: Default::default(),
                speed: 30,
                ability: Default::default(),
            }
        }
    }

    // - Race -

    impl Default for Race {
        fn default() -> Self {
            Self {
                name: "Unknown".to_string(),
            }
        }
    }

    // - Class -

    impl Default for Class {
        fn default() -> Self {
            Self {
                name: "Unknown".to_string(),
                level: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ability_score_to_modifier() {
        for (score, expected_modifier) in [
            (1, -5),
            (2, -4),
            (3, -4),
            (4, -3),
            (5, -3),
            (6, -2),
            (7, -2),
            (8, -1),
            (9, -1),
            (10, 0),
            (11, 0),
            (12, 1),
            (13, 1),
            (14, 2),
            (15, 2),
            (16, 3),
            (17, 3),
            (18, 4),
            (19, 4),
            (20, 5),
            (21, 5),
            (22, 6),
            (23, 6),
            (24, 7),
            (25, 7),
            (26, 8),
            (27, 8),
            (28, 9),
            (29, 9),
            (30, 10),
        ] {
            assert_eq!(
                Ability::from_score(score).modifier(),
                expected_modifier,
                "Failed for score: {}",
                score
            );
        }
    }

    #[test]
    fn ability_score_to_modifier_down() {
        for (modifier, expected_score) in [
            (-5, 1),
            (-4, 2),
            (-3, 4),
            (-2, 6),
            (-1, 8),
            (0, 10),
            (1, 12),
            (2, 14),
            (3, 16),
            (4, 18),
            (5, 20),
            (6, 22),
            (7, 24),
            (8, 26),
            (9, 28),
            (10, 30),
        ] {
            assert_eq!(
                Ability::from_modifier_down(modifier).score(),
                expected_score,
                "Failed for modifier: {}",
                modifier
            );
        }
    }

    #[test]
    fn ability_score_to_modifier_up() {
        for (modifier, expected_score) in [
            (-5, 1),
            (-4, 3),
            (-3, 5),
            (-2, 7),
            (-1, 9),
            (0, 11),
            (1, 13),
            (2, 15),
            (3, 17),
            (4, 19),
            (5, 21),
            (6, 23),
            (7, 25),
            (8, 27),
            (9, 29),
            (10, 30),
        ] {
            assert_eq!(
                Ability::from_modifier_up(modifier).score(),
                expected_score,
                "Failed for modifier: {}",
                modifier
            );
        }
    }
}
