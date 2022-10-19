pub struct Building {
    kind: BuildingKind,
}
impl Building {
    pub fn new(kind: BuildingKind) -> Self {
        Self { kind }
    }

    pub fn update(&self) {}
}

pub enum BuildingKind {
    Mine,
}
