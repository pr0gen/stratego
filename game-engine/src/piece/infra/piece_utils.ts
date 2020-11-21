import { Piece, PieceType, create as pieceCreate, Color } from '../piece';

export function createPiecesForNormalGame(c: Color) : Piece[] {
	return [
		pieceCreate(PieceType.Bomb, c),
		pieceCreate(PieceType.Bomb, c),
		pieceCreate(PieceType.Bomb, c),
		pieceCreate(PieceType.Bomb, c),
		pieceCreate(PieceType.Bomb, c),
		pieceCreate(PieceType.Bomb, c),

		pieceCreate(PieceType.Marshal, c),
		
		pieceCreate(PieceType.General, c),

		pieceCreate(PieceType.Colonel, c),
		pieceCreate(PieceType.Colonel, c),

		pieceCreate(PieceType.Major, c),
		pieceCreate(PieceType.Major, c),
		pieceCreate(PieceType.Major, c),

		pieceCreate(PieceType.Captain, c),
		pieceCreate(PieceType.Captain, c),
		pieceCreate(PieceType.Captain, c),
		pieceCreate(PieceType.Captain, c),

		pieceCreate(PieceType.Lieutenant, c),
		pieceCreate(PieceType.Lieutenant, c),
		pieceCreate(PieceType.Lieutenant, c),
		pieceCreate(PieceType.Lieutenant, c),

		pieceCreate(PieceType.Sergeant, c),
		pieceCreate(PieceType.Sergeant, c),
		pieceCreate(PieceType.Sergeant, c),
		pieceCreate(PieceType.Sergeant, c),

		pieceCreate(PieceType.Miner, c),
		pieceCreate(PieceType.Miner, c),
		pieceCreate(PieceType.Miner, c),
		pieceCreate(PieceType.Miner, c),
		pieceCreate(PieceType.Miner, c),

		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),
		pieceCreate(PieceType.Scout, c),

		pieceCreate(PieceType.Spy, c),

		pieceCreate(PieceType.Flag, c),


	];
}




