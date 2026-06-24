use serde::{Deserialize, Serialize};

// Servait pour le status des retours de fight mais le state est plus logique. On tente avec le state. À supprimer si on ne l'utilise plus.
#[derive(Debug, PartialEq, Serialize, Deserialize)]

pub enum FighterStatus {
    COMBAT,
    ENTERFIGHT
}