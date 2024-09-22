#[derive(Debug, Clone)]
pub struct Tile {
    pub position: AxialCoord,
    pub walls: [bool; 6],
    pub visited: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

impl Tile {
    pub fn visit(&mut self) {
        self.visited = true
    }
}

impl AxialCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
}
