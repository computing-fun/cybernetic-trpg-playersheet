impl Default for super::PlayerSheet {
    fn default() -> Self {
        Self {
            name: "Hero".to_string(),
            race: Default::default(),
            class: Default::default(),
        }
    }
}

impl Default for super::Race {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
        }
    }
}

impl Default for super::Class {
    fn default() -> Self {
        Self {
            name: "Classless".to_string(),
        }
    }
}
