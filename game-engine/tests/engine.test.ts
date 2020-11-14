import { GameEngine } from '../src/engine';
import { Board, StrategoBoard } from '../src/board/board';
import { Case } from '../src/case';
import { Piece, PieceType } from '../src/piece/piece';

test('Should init game', () => {
	let p1 = {name: "Ephrimes"};
	let p2 = {name: "Rolfie"};

	let engine = new GameEngine(p1, p2, StrategoBoard.createEmptyStrategoBoard(10));

	let state: Case[][] = engine.getBoard()
	.state();

	expect(state[2][4].content.rank).toBe(PieceType.Null);
	expect(state[3][4].content.rank).toBe(PieceType.Null);
	expect(state[2][5].content.rank).toBe(PieceType.Null);
	expect(state[3][5].content.rank).toBe(PieceType.Null);
	expect(state[6][4].content.rank).toBe(PieceType.Null);
	expect(state[7][4].content.rank).toBe(PieceType.Null);
	expect(state[6][5].content.rank).toBe(PieceType.Null);
	expect(state[7][5].content.rank).toBe(PieceType.Null);
});

