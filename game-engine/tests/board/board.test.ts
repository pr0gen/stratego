import * as board from '../../src/board/board';
import { PieceType, create } from '../../src/piece/piece';

test('Should build a StrategoBoard', () => {
	let strategoBoard = new board.StrategoBoard(2);
	let state = strategoBoard.state();
	
	expect(state.length).toBe(2);
	expect(state[0].length).toBe(2);
});

test('Should place a piece in board', () => {
	let strategoBoard = new board.StrategoBoard(2);
	strategoBoard.placePiece(0, 0, create(PieceType.Bomb));

	let state = strategoBoard.state();

	let actualCase = state[0][0];
	let content = actualCase.content;

	expect(content.rank).toBe(PieceType.Bomb);
	expect(content.move).toEqual({min: 0, max:0});

	expect(actualCase.state).toBe(1);
	expect(actualCase.x).toBe(0);
	expect(actualCase.y).toBe(0);
});
