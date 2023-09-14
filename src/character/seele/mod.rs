use crate::{
    engine::Engine,
    listener::Listener,
    model::{
        AttackInfo, Element, Modifier, ModifierInfo, Property, PropertyInfo,
        PropertyType,
    },
};

pub const ID: &str = "seele";
pub const RESURGENCE: &str = "seele_resurgence";
pub const SPEED: &str = "seele_speed";

#[derive(Clone, Copy)]
pub struct Seele;

impl Seele {

    pub fn init(&self, engine: &mut Engine) {
        let listener = Listener {
            on_hit_start: Some(|_, _| info!("Hit Start Callback WOOOOOHOOOOO")),
            ..Default::default()
        };

        engine.add_listener(listener);
    }

    pub fn attack(&self, target: &str, engine: &mut Engine) {
        engine.attack(AttackInfo {
            name: "seele_attack".to_string(),
            source: ID.to_string(),
            target: target.to_string(),
            damage_type: Element::Quantum,
            energy_gain: 20,
            hit_ratio: vec![0.3, 0.7],
        });
    }

    pub fn skill(&self, target: &str, engine: &mut Engine) {
        engine.attack(AttackInfo {
            name: "seele_skill".to_string(),
            source: ID.to_string(),
            target: target.to_string(),
            damage_type: Element::Quantum,
            energy_gain: 30,
            hit_ratio: vec![0.2, 0.1, 0.1, 0.6],
        });

        engine.add_property(PropertyInfo {
            target: ID.to_string(),
            property: Property {
                id: SPEED.to_string(),
                property_type: PropertyType::SpeedPercent(0.25),
                duration: Some(2),
            },
        });
    }

    pub fn ultra(&self, target: &str, engine: &mut Engine) {
        engine.add_modifier(ModifierInfo {
            target: ID.to_string(),
            modifier: Modifier {
                id: RESURGENCE.to_string(),
                duration: Some(1),
            },
        });

        engine.attack(AttackInfo {
            name: "seele_ultra".to_string(),
            source: ID.to_string(),
            target: target.to_string(),
            damage_type: Element::Quantum,
            energy_gain: 5,
            hit_ratio: vec![1.0],
        });
    }
}
