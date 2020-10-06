import { Piece } from './piece/piece';
import { Case, CaseState } from './case';

export interface Board {

	state(): Board;

	moveTo:(c: Case) => Piece;
}

