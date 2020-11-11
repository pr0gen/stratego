import { Piece, PieceType, create as pieceCreate } from '../piece';

export function createAllPiece() : Piece[] {
	return [
		pieceCreate(PieceType.Bomb),
		pieceCreate(PieceType.Bomb),
		pieceCreate(PieceType.Bomb),
		pieceCreate(PieceType.Bomb),
		pieceCreate(PieceType.Bomb),
		pieceCreate(PieceType.Bomb),

		pieceCreate(PieceType.Marshal),
		
		pieceCreate(PieceType.General),

		pieceCreate(PieceType.Colonel),
		pieceCreate(PieceType.Colonel),

		pieceCreate(PieceType.Major),
		pieceCreate(PieceType.Major),
		pieceCreate(PieceType.Major),

		pieceCreate(PieceType.Captain),
		pieceCreate(PieceType.Captain),
		pieceCreate(PieceType.Captain),
		pieceCreate(PieceType.Captain),

		pieceCreate(PieceType.Lieutenant),
		pieceCreate(PieceType.Lieutenant),
		pieceCreate(PieceType.Lieutenant),
		pieceCreate(PieceType.Lieutenant),

		pieceCreate(PieceType.Sergeant),
		pieceCreate(PieceType.Sergeant),
		pieceCreate(PieceType.Sergeant),
		pieceCreate(PieceType.Sergeant),

		pieceCreate(PieceType.Miner),
		pieceCreate(PieceType.Miner),
		pieceCreate(PieceType.Miner),
		pieceCreate(PieceType.Miner),
		pieceCreate(PieceType.Miner),

		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),
		pieceCreate(PieceType.Scout),

		pieceCreate(PieceType.Spy),

		pieceCreate(PieceType.Flag),


	];
}




