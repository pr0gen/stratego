import {Ok, Err, Result} from 'ts-results';

import { Piece } from './piece/piece';
import { Case, CaseState } from './case';
import { StrategoError, MoveError } from './error/error';


export interface Board {

	board: Case[][];

	state(): Case[][];

	place(c: Case, p: Piece): Result<Piece, StrategoError>;
}

export class StrategoBoard implements Board {

	board: Case[][];

	constructor() {
		this.board = new Array(new Array(10));
	}

	state(): Case[][] {
		return this.board;
	}

	place(c: Case, p: Piece): Result<Piece, StrategoError> {
		let actualCase = this.board[c.x][c.y];
		if (actualCase.state == CaseState.Empty) {
			this.board[c.x][c.y] = {state: CaseState.Full, x: c.x, y: c.y, content: p};
			return Ok(p);
		}
		return Err(new MoveError(c, p));	
	}

}


