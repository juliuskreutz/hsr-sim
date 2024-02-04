#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Target {
    Character(String),
    Enemy(String),
}
