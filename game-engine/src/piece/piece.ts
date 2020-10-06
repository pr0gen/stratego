import { Move, AvailableMoves, create as moveCreate} from './move';

export interface Piece {
	move: Move,
	rank: number,
}

export enum PieceType {
	Bomb = -2,
	Marshal = 10,
	General = 9,
	Colonel = 8,
	Major = 7,
	Captain = 6,
	Lieutenant = 5,
	Sergeant = 4,
	Miner = 3,
	Scout = 2,
	Spy = 1,
	Flag = -1
}

export function create(type: PieceType) : Piece {
	switch(type) {
		case PieceType.Bomb:
			return {
			move: moveCreate(AvailableMoves.Immovable),
			rank: PieceType.Bomb
		};
		case PieceType.Marshal:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Marshal
		};
		case PieceType.General:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.General
		};
		case PieceType.Colonel:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Colonel
		};
		case PieceType.Major:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Major
		};
		case PieceType.Captain:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Captain
		};
		case PieceType.Lieutenant:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Lieutenant
		};
		case PieceType.Sergeant:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Sergeant
		};
		case PieceType.Miner:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Miner
		};
		case PieceType.Scout:
			return {
			move: moveCreate(AvailableMoves.Scout),
			rank: PieceType.Scout
		};
		case PieceType.Spy:
			return {
			move: moveCreate(AvailableMoves.Normal),
			rank: PieceType.Spy
		};
		case PieceType.Flag:
			return {
			move: moveCreate(AvailableMoves.Immovable),
			rank: PieceType.Flag
		};
	}
}

