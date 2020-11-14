import * as engine from '../src/engine_utils';
import { Board, StrategoBoard } from '../src/board/board';
import { Case, CaseState, createEmpty, createUnreachable, create as createCase } from '../src/case';
import { InitGameError } from '../src/error/error';

test('Should not verify board integrity cause to small', () => {
	let newBoard: Case[][] = [
		[createEmpty(0, 0), createEmpty(0, 1)],
		[createEmpty(1, 0), createUnreachable(1, 1)]
	];
	let board: Board = new StrategoBoard(newBoard);
	let res = engine.verifyIntegrityBoard(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("Board is not official, GO OUT OF THERE !!"));
});

test('Should not verify board integrity cause lakes are empty', () => {
		let newBoard: Case[][] = new Array(10);

		for(let i = 0; i < 10; i++){
			newBoard[i] = new Array(10);
			for(let j = 0; j < 10; j++){
				newBoard[i][j] = createCase(CaseState.Full ,i ,j ,{move:{min:0, max:1}, rank: 1});
			}
		}
	
	let board: Board = new StrategoBoard(newBoard);

	let res = engine.verifyIntegrityBoard(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("You can not place pieces in lakes, please check again"));
});

