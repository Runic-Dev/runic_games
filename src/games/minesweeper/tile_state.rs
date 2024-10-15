#[derive(Clone, PartialEq, Default)]
pub struct TileState {
    pub tile_type: TileType,
    pub is_dug: bool,
    pub number: usize,
    pub is_flagged: bool,
}

#[derive(Clone, PartialEq)]
pub enum TileType {
    Number { local_mines: usize },
    Bomb,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Number { local_mines: 0 }
    }
}
