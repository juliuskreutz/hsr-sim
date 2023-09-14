use crate::{character::seele::Seele, engine::Engine};

pub enum CharacterType {
    Seele(Seele),
}

pub enum EnemyType {
    Dummy,
}

pub enum TargetType {
    Character(CharacterType),
    Enemy(EnemyType),
}

pub struct Target {
    pub id: String,
    pub hit_points: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64,
    pub energy: usize,
    pub target_type: TargetType,
    pub critical_rate: f64,
    pub critical_damage: f64,
}

pub fn init(source: &str, engine: &mut Engine) {
    if let TargetType::Character(CharacterType::Seele(seele)) = engine.targets[source].target_type {
        seele.init(engine)
    }
}

pub fn attack(source: &str, target: &str, engine: &mut Engine) {
    if let TargetType::Character(CharacterType::Seele(seele)) = engine.targets[source].target_type {
        seele.attack(target, engine)
    }
}

pub fn skill(source: &str, target: &str, engine: &mut Engine) {
    if let TargetType::Character(CharacterType::Seele(seele)) = engine.targets[source].target_type {
        seele.skill(target, engine)
    }
}

pub fn ultra(source: &str, target: &str, engine: &mut Engine) {
    if let TargetType::Character(CharacterType::Seele(seele)) = engine.targets[source].target_type {
        seele.ultra(target, engine)
    }
}
