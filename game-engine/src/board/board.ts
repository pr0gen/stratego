import { Ok, Err, Result } from 'ts-results';
import { Piece, PieceType, create as createPiece } from '../piece/piece';
import { Case, CaseState, create as createCase } from '../case';
import { StrategoError, MoveError, PlacementError } from '../error/error';

export interface Board {

	board: Case[][];

	state(): Case[][];

	move(c: Case, p: Piece): Result<Piece, StrategoError>;
}

export class StrategoBoard implements Board {

	board: Case[][];

	public constructor(size: number) {
		this.board = new Array(size);
		for(var i: number = 0; i < size; i++) {
			this.board[i] = new Array(size);
			for(var j: number = 0; j < size; j++) {
				this.board[i][j] = createCase(CaseState.Empty, i, j, createPiece(PieceType.Null));
			}
		}
	}

	public placePiece(x: number, y: number, p: Piece): Result<Piece, StrategoError> {
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

	move(c: Case, p: Piece): Result<Piece, StrategoError> {
		let actualCase = this.board[c.x][c.y];
		if (actualCase.state == CaseState.Empty) {
			this.board[c.x][c.y] = {state: CaseState.Full, x: c.x, y: c.y, content: p};
			return Ok(p);
		}
		return Err(new MoveError(c, p));	
	}

}

