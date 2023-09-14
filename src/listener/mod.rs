use crate::engine::Engine;

pub struct HitStart {
    pub source: String,
    pub target: String,
}

pub struct HitEnd {
    pub source: String,
    pub target: String,
}

#[derive(Default, Clone, Copy)]
pub struct Listener {
    pub on_hit_start: Option<fn(&mut HitStart, &mut Engine)>,
    pub on_hit_end: Option<fn(&mut HitEnd, &mut Engine)>,
}
