#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardPiece {
    Empty,
    Hero,
    Wumpus,
    Bats,
    Arrow,
    Hole,
}
pub enum ArrowTarget {
    Wumpus,
    Bats,
    None,
}
