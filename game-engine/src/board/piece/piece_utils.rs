use std::collections::HashMap;

use super::{Color, Piece, PieceType};

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
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Bomb, Box::new(color)),
        Piece::new(PieceType::Marshal, Box::new(color)),
        Piece::new(PieceType::General, Box::new(color)),
        Piece::new(PieceType::Colonel, Box::new(color)),
        Piece::new(PieceType::Colonel, Box::new(color)),
        Piece::new(PieceType::Major, Box::new(color)),
        Piece::new(PieceType::Major, Box::new(color)),
        Piece::new(PieceType::Major, Box::new(color)),
        Piece::new(PieceType::Captain, Box::new(color)),
        Piece::new(PieceType::Captain, Box::new(color)),
        Piece::new(PieceType::Captain, Box::new(color)),
        Piece::new(PieceType::Captain, Box::new(color)),
        Piece::new(PieceType::Lieutenant, Box::new(color)),
        Piece::new(PieceType::Lieutenant, Box::new(color)),
        Piece::new(PieceType::Lieutenant, Box::new(color)),
        Piece::new(PieceType::Lieutenant, Box::new(color)),
        Piece::new(PieceType::Sergeant, Box::new(color)),
        Piece::new(PieceType::Sergeant, Box::new(color)),
        Piece::new(PieceType::Sergeant, Box::new(color)),
        Piece::new(PieceType::Sergeant, Box::new(color)),
        Piece::new(PieceType::Miner, Box::new(color)),
        Piece::new(PieceType::Miner, Box::new(color)),
        Piece::new(PieceType::Miner, Box::new(color)),
        Piece::new(PieceType::Miner, Box::new(color)),
        Piece::new(PieceType::Miner, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Scout, Box::new(color)),
        Piece::new(PieceType::Spy, Box::new(color)),
        Piece::new(PieceType::Flag, Box::new(color)),
    ];
}
