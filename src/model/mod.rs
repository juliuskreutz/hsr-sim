pub enum Element {
    Quantum,
}

pub struct AttackInfo {
    pub name: String,
    pub source: String,
    pub target: String,
    pub damage_type: Element,
    pub energy_gain: usize,
    pub hit_ratio: Vec<f64>,
}

pub struct Modifier {
    pub id: String,
    pub duration: Option<usize>,
}

pub struct ModifierInfo {
    pub target: String,
    pub modifier: Modifier,
}

pub enum PropertyType {
    SpeedPercent(f64),
}

pub struct Property {
    pub id: String,
    pub property_type: PropertyType,
    pub duration: Option<usize>,
}

pub struct PropertyInfo {
    pub target: String,
    pub property: Property,
}
