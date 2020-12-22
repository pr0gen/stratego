import { PieceType } from '../piece';

export function listAllPieces(): Map<PieceType, number> {
    return new Map([
        [PieceType.Bomb, 6],
        [PieceType.Marshal, 1],
        [PieceType.General, 1],
        [PieceType.Colonel, 2],
        [PieceType.Major, 3],
        [PieceType.Captain, 4],
        [PieceType.Lieutenant, 4],
        [PieceType.Sergeant, 4],
        [PieceType.Miner, 5],
        [PieceType.Scout, 8],
        [PieceType.Spy, 1],
        [PieceType.Flag, 1],
    ]);
}




