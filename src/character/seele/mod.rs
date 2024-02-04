use crate::{
    engine::Engine,
    model::{AttackInfo, Element},
    target::Target,
};

pub const ID: &str = "seele";
const BASIC: &str = "seele_basic";
const SKILL: &str = "seele_skill";
const ULTRA: &str = "seele_ultra";

pub fn basic(target: Target, engine: &mut Engine) {
    let Target::Enemy(enemy) = target else {
        unreachable!();
    };

    engine.attack(AttackInfo {
        name: BASIC.to_string(),
        source: ID.to_string(),
        target: enemy.to_string(),
        damage_type: Element::Quantum,
        energy_gain: 20,
        hit_ratio: vec![0.3, 0.7],
    });
}

pub fn skill(target: Target, engine: &mut Engine) {
    let Target::Enemy(enemy) = target else {
        unreachable!();
    };

    engine.attack(AttackInfo {
        name: SKILL.to_string(),
        source: ID.to_string(),
        target: enemy.to_string(),
        damage_type: Element::Quantum,
        energy_gain: 20,
        hit_ratio: vec![0.3, 0.7],
    });
}

pub fn ultra(target: Target, engine: &mut Engine) {
    let Target::Enemy(enemy) = target else {
        unreachable!();
    };
}
