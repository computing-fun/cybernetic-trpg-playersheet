pub mod default;
pub mod io;
mod create_or_open_dialog_app;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerSheet {
    pub name: String,
    pub race: Race,
    pub class: Class,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Race {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub name: String,
}
