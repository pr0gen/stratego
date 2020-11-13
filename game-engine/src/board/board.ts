import { Ok, Err, Result } from 'ts-results';
import { Piece, PieceType, create as createPiece } from '../piece/piece';
import { Case, CaseState, create as createCase } from '../case';
import { StrategoError, MoveError, PlacementError } from '../error/error';

export interface Coordinate {
	x: number,
	y: number
}

export interface Board {

	board: Case[][];

	placePiece(x: number, y: number, p: Piece): Result<Piece, StrategoError>;

	state(): Case[][];

	move(c: Case, to: Coordinate): Result<Case, StrategoError>;

}

export class StrategoBoard implements Board {

	board: Case[][];

	public static createEmptyStrategoBoard(size: number): Board {
		let board = new Array(size);
		for(var i: number = 0; i < size; i++) {
			board[i] = new Array(size);
			for(var j: number = 0; j < size; j++) {
				board[i][j] = createCase(CaseState.Empty, i, j, createPiece(PieceType.Null));
			}
		}
		return new StrategoBoard(board);
	}

	public static create10x10StrategoBoard(): Board {
		//TODO
		return new StrategoBoard([]);
	}

	public constructor(board: Case[][]) {
		this.board = board;
	}

	placePiece(x: number, y: number, p: Piece): Result<Piece, StrategoError> {
		let actualCase = this.board[x][y];
		if (actualCase.state == CaseState.Empty) {
			this.board[x][y] = createCase(CaseState.Full, x, y, p);
			return Ok(p);
		}

		return Err(new PlacementError(x, y, p));
	}

	state(): Case[][] {
		return this.board;
	}

	move(c: Case, to: Coordinate): Result<Case, StrategoError> {
		let aimCase = this.board[to.x][to.y];
		switch (aimCase.state) {
			case CaseState.Empty: {
				let newCase = createCase(CaseState.Full, to.x, to.y, c.content);
				this.board[to.x][to.y] = newCase;
				return Ok(newCase);
			}
			case CaseState.Full: {
				//TODO
			}
			case CaseState.Unreachable:  {
				return Err(new MoveError(c, to));
			}
			default: {
				return Err(new MoveError(c, to));
			}
		}
	}

}

