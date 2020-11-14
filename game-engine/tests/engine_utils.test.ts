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
	let res = engine.verifyBoardIntegrity(board);

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

	let res = engine.verifyBoardIntegrity(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("You can not place pieces in lakes, please check again"));
});

test('Should check only 2 rows in middle are empty', () => {
		let newBoard: Case[][] = new Array(10);

		for(let i = 0; i < 10; i++){
			newBoard[i] = new Array(10);
			for(let j = 0; j < 10; j++){
				newBoard[i][j] = createCase(CaseState.Full ,i ,j ,{move:{min:0, max:1}, rank: 1});
			}
		}

		newBoard[4][2] = createUnreachable(4, 2);
		newBoard[4][3] = createUnreachable(4, 3);
		newBoard[5][2] = createUnreachable(5, 2);
		newBoard[5][3] = createUnreachable(5, 3);
		newBoard[4][6] = createUnreachable(4, 6);
		newBoard[4][7] = createUnreachable(4, 7);
		newBoard[5][6] = createUnreachable(5, 6);
		newBoard[5][7] = createUnreachable(7, 5);
	
	let board: Board = new StrategoBoard(newBoard);
	console.log(board.display());


	let res = engine.verifyBoardIntegrity(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("The 2 rows in the middle must be empty, :("));
});

//test('Should verify board integrity', () => {
	//let board: Board = StrategoBoard.create10x10StrategoBoard();

	//let res = engine.verifyBoardIntegrity(board);
	//expect(res.ok).toBe(true);
		
//});

