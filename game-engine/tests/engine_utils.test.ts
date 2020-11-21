import * as Engine from '../src/engine_utils';
import { Board, StrategoBoard } from '../src/board/board';
import { Case, CaseState, createEmpty, createUnreachable, create as createCase } from '../src/case';
import { InitGameError } from '../src/error/error';
import { Color } from '../src/piece/piece';

test('Should not verify board integrity cause to small', () => {
	let newBoard: Case[][] = [
		[createEmpty(0, 0), createEmpty(0, 1)],
		[createEmpty(1, 0), createUnreachable(1, 1)]
	];
	let board: Board = new StrategoBoard(newBoard);
	let res = Engine.verifyBoardIntegrity(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("Board is not official, GO OUT OF THERE !!"));
});

test('Should not verify board integrity cause lakes are empty', () => {
	let newBoard: Case[][] = createStrategoBoard();
	
	newBoard[4][2] = createCase( CaseState.Full, 4, 2, {move:{min:0, max:1}, rank: 1, color: Color.Blue});
	let board: Board = new StrategoBoard(newBoard);

	let res = Engine.verifyBoardIntegrity(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("You can not place pieces in lakes, please check again"));
});

test('Should check only 2 rows in middle are empty', () => {
		let newBoard: Case[][] = new Array(10);

		for(let i = 0; i < 10; i++){
			newBoard[i] = new Array(10);
			for(let j = 0; j < 10; j++){
				newBoard[i][j] = createCase(
					CaseState.Full,
					i,
					j,
					{move:{min:0, max:1}, rank: 1, color: Color.Blue}
				);
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
	//console.log("2 rows\n" + board.display());
	let res = Engine.verifyBoardIntegrity(board);

	expect(res.err).toBe(true);
	expect(res.val).toStrictEqual(new InitGameError("The 2 rows in the middle must be empty, :("));
});

test('Should check players have placed theirs pieces in the 4 rows', () => {
		let newBoard: Case[][] = new Array(10);

		for(let i = 0; i < 10; i++){
			newBoard[i] = new Array(10);
			for(let j = 0; j < 10; j++){
				if (i < 4) {
					newBoard[i][j] = createCase(
						CaseState.Full,
						i,
						j,
						{move:{min:0, max:1}, rank: 1, color: Color.Blue}
					);
				} else if (i > 5){
					newBoard[i][j] = createCase(
						CaseState.Full,
						i,
						j,
						{move:{min:0, max:1}, rank: 1, color: Color.Red}
					);
				} else {
					newBoard[i][j] = createEmpty(i, j);
				}
			}
		}

		// Placing a piece to the bad place
		newBoard[0][4] = createCase(CaseState.Full, 0, 4, {move:{min:0, max:1}, rank: 1, color: Color.Red});
		
		newBoard[4][2] = createUnreachable(4, 2);
		newBoard[4][3] = createUnreachable(4, 3);
		newBoard[5][2] = createUnreachable(5, 2);
		newBoard[5][3] = createUnreachable(5, 3);
		newBoard[4][6] = createUnreachable(4, 6);
		newBoard[4][7] = createUnreachable(4, 7);
		newBoard[5][6] = createUnreachable(5, 6);
		newBoard[5][7] = createUnreachable(7, 5);

		let board: Board = new StrategoBoard(newBoard);
		//console.log("4 rows\n" + board.display());
		let res = Engine.verifyBoardIntegrity(board);


		expect(res.err).toBe(true);
		expect(res.val).toStrictEqual(new InitGameError("Players must place theirs pieces in the right place !"));

});

test('Should verify board integrity', () => {
	let board: Board = new StrategoBoard(createStrategoBoard());
	let res = Engine.verifyBoardIntegrity(board);

	expect(res.ok).toBe(true);
});


//Good board
function createStrategoBoard(): Case[][] {
	let newBoard: Case[][] = new Array(10);

	for(let i = 0; i < 10; i++){
		newBoard[i] = new Array(10);
		for(let j = 0; j < 10; j++){
			if (i < 4) {
				newBoard[i][j] = createCase(
					CaseState.Full,
					i,
					j,
					{move:{min:0, max:1}, rank: 1, color: Color.Blue}
				);
			} else if (i > 5){
				newBoard[i][j] = createCase(
					CaseState.Full,
					i,
					j,
					{move:{min:0, max:1}, rank: 1, color: Color.Red}
				);
			} else {
				newBoard[i][j] = createEmpty(i, j);
			}
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

	return newBoard;
}

