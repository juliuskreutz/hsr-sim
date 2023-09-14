use std::collections::HashMap;

use priority_queue::PriorityQueue;

use crate::{
    character::seele,
    listener::{HitStart, Listener},
    model::{AttackInfo, Modifier, ModifierInfo, Property, PropertyInfo, PropertyType},
    target::{self, CharacterType, EnemyType, Target, TargetType},
};

pub struct Order {
    pub source: String,
    pub action_value: f64,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.action_value.partial_cmp(&self.action_value)
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.action_value.total_cmp(&self.action_value)
    }
}

pub struct Engine {
    pub listeners: Vec<Listener>,
    pub targets: HashMap<String, Target>,
    pub action_order: PriorityQueue<String, i32>,
    pub modifiers: HashMap<String, HashMap<String, Modifier>>,
    pub properties: HashMap<String, HashMap<String, Property>>,
}

impl Engine {
    pub fn new() -> Self {
        let listeners = Vec::new();

        let mut targets = HashMap::new();

        targets.insert(
            seele::ID.to_string(),
            Target {
                id: seele::ID.to_string(),
                hit_points: 3000.0,
                attack: 3000.0,
                defense: 0.0,
                speed: 115.0,
                energy: 60,
                critical_rate: 0.0,
                critical_damage: 0.0,
                target_type: TargetType::Character(CharacterType::Seele(seele::Seele)),
            },
        );
        targets.insert(
            "dummy".to_string(),
            Target {
                id: "dummy".to_string(),
                hit_points: 9999999999.0,
                attack: 0.0,
                defense: 0.0,
                speed: 0.0,
                energy: 0,
                critical_rate: 0.0,
                critical_damage: 0.0,
                target_type: TargetType::Enemy(EnemyType::Dummy),
            },
        );

        let mut action_order = PriorityQueue::new();
        action_order.push(seele::ID.to_string(), (10000.0 / targets[seele::ID].speed) as i32);

        let modifiers = HashMap::new();
        let properties = HashMap::new();

        Self {
            listeners,
            targets,
            action_order,
            modifiers,
            properties,
        }
    }

    pub fn run(&mut self) {
        let sources: Vec<_> = self.targets.keys().cloned().collect();

        for source in sources {
            target::init(&source, self);
        }

        let mut cycle = 0;
        let mut action_value: i32 = 150;

        loop {
            let (source, source_action_value) = self.action_order.pop().unwrap();
            action_value -= source_action_value;

            for (_, av) in self.action_order.iter_mut() {
                *av -= source_action_value;
            }

            if action_value < 0 {
                cycle += 1;

                if cycle == 8 {
                    break;
                }

                action_value += 100;
            }

            self.action(&source);
        }
    }

    pub fn action(&mut self, source: &str) {
        if self.targets[source].energy >= 120 {
            self.targets
                .entry(source.to_string())
                .and_modify(|t| t.energy = 0);

            target::ultra(source, "dummy", self);
        }

        target::skill(source, "dummy", self);

        let mut speed = self.targets[source].speed;

        if let Some(properties) = self.properties.get(source) {
            for property in properties.values() {
                if let PropertyType::SpeedPercent(speed_percent) = property.property_type {
                    speed *= 1.0 + speed_percent;
                }
            }
        }

        info!("{source} has speed {speed}");

        self.action_order
            .push(source.to_string(), (10000.0 / speed) as i32);

        if let Some(modifiers) = self.modifiers.get_mut(source) {
            for modifier in modifiers.values_mut() {
                if let Some(duration) = &mut modifier.duration {
                    *duration -= 1;
                }
            }

            modifiers.retain(|_, modifier| {
                let remove = modifier.duration == Some(0);

                if remove {
                    info!("Removing modifier from {source} {}", modifier.id);
                }

                !remove
            });
        }

        if let Some(properties) = self.properties.get_mut(source) {
            for property in properties.values_mut() {
                if let Some(duration) = &mut property.duration {
                    *duration -= 1;
                }
            }

            properties.retain(|_, property| {
                let remove = property.duration == Some(0);

                if remove {
                    info!("Removing property from {source} {}", property.id);
                }

                !remove
            });
        }
    }

    pub fn attack(&mut self, attack_info: AttackInfo) {
        info!("{} attacks with {}", attack_info.source, attack_info.name);

        for hit_ratio in attack_info.hit_ratio {
            let mut hit_start = HitStart {
                source: attack_info.source.clone(),
                target: attack_info.target.clone(),
            };

            self.hit_start(&mut hit_start);

            self.targets
                .get_mut(&attack_info.target)
                .unwrap()
                .hit_points -= self.targets[&attack_info.source].attack * hit_ratio;
        }

        self.targets
            .entry(attack_info.source)
            .and_modify(|target| target.energy += attack_info.energy_gain);
    }

    pub fn add_modifier(&mut self, modifier_info: ModifierInfo) {
        info!(
            "Add modifier for {} {}",
            modifier_info.target, modifier_info.modifier.id
        );

        self.modifiers
            .entry(modifier_info.target)
            .or_default()
            .insert(
                modifier_info.modifier.id.to_string(),
                modifier_info.modifier,
            );
    }

    pub fn add_property(&mut self, property_info: PropertyInfo) {
        info!(
            "Add property for {} {}",
            property_info.target, property_info.property.id
        );

        self.properties
            .entry(property_info.target)
            .or_default()
            .insert(
                property_info.property.id.to_string(),
                property_info.property,
            );
    }

    fn hit_start(&mut self, hit_start: &mut HitStart) {
        for listener in self.listeners.clone() {
            if let Some(on_hit_start) = listener.on_hit_start {
                on_hit_start(hit_start, self);
            }
        }
    }

    pub fn add_listener(&mut self, listener: Listener) {
        self.listeners.push(listener);
    }
}
