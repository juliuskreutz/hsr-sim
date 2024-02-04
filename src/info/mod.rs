pub struct Info {
    pub attack: f64,
    pub hit_points: f64,
    pub defense: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub speed: f64,
    pub effects: Vec<Effect>,
}

pub enum Effect {}
