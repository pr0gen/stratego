import { AvailableMoves, create as createMove } from './piece/move';
import { Piece, PieceType } from './piece/piece';

export interface Case {
	state: CaseState,
	x: number,
	y: number,
	content: Piece
}

export enum CaseState {
	Unreachable = -1,
	Empty = 0,
	Full = 1
}

export function display(c: Case): string {
	return "{" + c.x + "," + c.y + "}" + c.content.rank.toString();
}

export function createUnreachable(x: number, y: number): Case {
	return create(
		CaseState.Unreachable,
		x, y,
		{move: createMove(AvailableMoves.Immovable), rank:PieceType.Null}
	);
}

export function createEmpty(x: number, y: number): Case {
	return create(
		CaseState.Empty,
		x, y,
		{move: createMove(AvailableMoves.Immovable), rank:PieceType.Null}
	);
}
export function create(state: CaseState, 
		      x: number,
		      y: number,
		      content: Piece): Case {

	return {state, x, y, content};
}
