import { Piece } from './piece/piece';

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

export function create(state: CaseState, 
		      x: number,
		      y: number,
		      content: Piece): Case {

	return {state, x, y, content};
}
