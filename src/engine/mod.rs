use std::{cmp::Reverse, collections::HashMap};

use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;

use crate::{
    character::{Character, CharacterType},
    enemy::Enemy,
    info::Info,
    model::AttackInfo,
    target::Target,
};

pub struct Engine {
    pub characters: HashMap<String, Character>,
    pub enemies: HashMap<String, Enemy>,
    pub action_order: PriorityQueue<Target, Reverse<OrderedFloat<f64>>>,
    pub action_value: f64,
    pub cycle: usize,
}

impl Engine {
    pub fn new() -> Self {
        let mut characters = HashMap::new();
        characters.insert(
            "seele".to_string(),
            Character {
                id: "seele".to_string(),
                character_type: CharacterType::Seele,
                eidolon: 0,
                energy: 0,
                info: Info {
                    attack: 3000.0,
                    hit_points: 3000.0,
                    defense: 0.0,
                    crit_chance: 0.7,
                    crit_damage: 2.0,
                    speed: 200.0,
                    effects: Vec::new(),
                },
            },
        );

        let mut enemies = HashMap::new();
        enemies.insert(
            "dummy".to_string(),
            Enemy {
                id: "dummy".to_string(),
                info: Info {
                    attack: 0.0,
                    hit_points: 100_000_000.0,
                    defense: 0.0,
                    crit_chance: 0.0,
                    crit_damage: 0.0,
                    speed: 134.0,
                    effects: Vec::new(),
                },
            },
        );
        enemies.insert(
            "dummy2".to_string(),
            Enemy {
                id: "dummy".to_string(),
                info: Info {
                    attack: 0.0,
                    hit_points: 100_000_000.0,
                    defense: 0.0,
                    crit_chance: 0.0,
                    crit_damage: 0.0,
                    speed: 187.0,
                    effects: Vec::new(),
                },
            },
        );

        let mut action_order = PriorityQueue::new();

        for character in characters.values() {
            action_order.push(
                Target::Character(character.id.clone()),
                Reverse(OrderedFloat(10000.0 / character.info.speed)),
            );
        }

        for enemy in enemies.values() {
            action_order.push(
                Target::Enemy(enemy.id.clone()),
                Reverse(OrderedFloat(10000.0 / enemy.info.speed)),
            );
        }

        Self {
            characters,
            enemies,
            action_order,
            action_value: 150.0,
            cycle: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let (source, source_action_value) = self.action_order.pop().unwrap();

            self.action_value -= source_action_value.0 .0;

            for (_, action_value) in self.action_order.iter_mut() {
                action_value.0 -= source_action_value.0;
            }

            if self.action_value < 0.0 {
                self.cycle += 1;

                if self.cycle == 7 {
                    break;
                }

                self.action_value += 100.0;
            }

            self.action(&source);
        }
    }

    pub fn attack(&mut self, attack_info: AttackInfo) {}

    fn action(&mut self, source: &Target) {
        match source {
            Target::Character(id) => self.character_action(id),
            Target::Enemy(id) => self.enemy_action(id),
        }
    }

    fn character_action(&mut self, id: &str) {
        crate::character::basic(id, Target::Enemy("dummy".to_string()), self);

        let character = &self.characters[id];
        self.action_order.push(
            Target::Character(character.id.clone()),
            Reverse(OrderedFloat(10000.0 / character.info.speed)),
        );
    }

    fn enemy_action(&mut self, id: &str) {
        let enemy = &self.enemies[id];

        self.action_order.push(
            Target::Enemy(enemy.id.clone()),
            Reverse(OrderedFloat(10000.0 / enemy.info.speed)),
        );
    }
}
