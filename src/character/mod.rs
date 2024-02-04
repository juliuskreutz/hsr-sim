pub mod seele;

use crate::{engine::Engine, info::Info, target::Target};

pub enum CharacterType {
    Seele,
}

pub struct Character {
    pub id: String,
    pub character_type: CharacterType,
    pub eidolon: usize,
    pub energy: usize,
    pub info: Info,
}

pub fn basic(source: &str, target: Target, engine: &mut Engine) {
    match engine.characters[source].character_type {
        CharacterType::Seele => seele::basic(target, engine),
    }
}

pub fn skill(source: &str, target: Target, engine: &mut Engine) {
    match engine.characters[source].character_type {
        CharacterType::Seele => seele::skill(target, engine),
    }
}

pub fn ultra(source: &str, target: Target, engine: &mut Engine) {
    match engine.characters[source].character_type {
        CharacterType::Seele => seele::ultra(target, engine),
    }
}
