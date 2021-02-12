use std::collections::HashMap;

use crate::board::piece::{Color, Piece, PieceType};

pub fn list_all_pieces() -> HashMap<PieceType, i8> {
    [
        (PieceType::Bomb, 6),
        (PieceType::Marshal, 1),
        (PieceType::General, 1),
        (PieceType::Colonel, 2),
        (PieceType::Major, 3),
        (PieceType::Captain, 4),
        (PieceType::Lieutenant, 4),
        (PieceType::Sergeant, 4),
        (PieceType::Miner, 5),
        (PieceType::Scout, 8),
        (PieceType::Spy, 1),
        (PieceType::Flag, 1),
    ]
    .iter()
    .cloned()
    .collect()
}

pub fn list_of_all_pieces(color: Color) -> Vec<Piece> {
    return vec![
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Bomb, color),
        Piece::new(PieceType::Marshal, color),
        Piece::new(PieceType::General, color),
        Piece::new(PieceType::Colonel, color),
        Piece::new(PieceType::Colonel, color),
        Piece::new(PieceType::Major, color),
        Piece::new(PieceType::Major, color),
        Piece::new(PieceType::Major, color),
        Piece::new(PieceType::Captain, color),
        Piece::new(PieceType::Captain, color),
        Piece::new(PieceType::Captain, color),
        Piece::new(PieceType::Captain, color),
        Piece::new(PieceType::Lieutenant, color),
        Piece::new(PieceType::Lieutenant, color),
        Piece::new(PieceType::Lieutenant, color),
        Piece::new(PieceType::Lieutenant, color),
        Piece::new(PieceType::Sergeant, color),
        Piece::new(PieceType::Sergeant, color),
        Piece::new(PieceType::Sergeant, color),
        Piece::new(PieceType::Sergeant, color),
        Piece::new(PieceType::Miner, color),
        Piece::new(PieceType::Miner, color),
        Piece::new(PieceType::Miner, color),
        Piece::new(PieceType::Miner, color),
        Piece::new(PieceType::Miner, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Scout, color),
        Piece::new(PieceType::Spy, color),
        Piece::new(PieceType::Flag, color),
    ];
}
