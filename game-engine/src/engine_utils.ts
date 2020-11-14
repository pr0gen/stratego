import { Board } from './board/board';
import { Player } from './player/player';
import { Err, Ok, Result } from 'ts-results';
import { InitGameError, StrategoError } from './error/error';
import { PieceType } from './piece/piece';
import { Case } from './case';

export function gameIsOver(board: Board, player: Player): boolean {
	return canNotAttack(board, player) && canNotMove(board, player);
}

function canNotMove(board: Board, player: Player): boolean {
	let actualBoard: Case[][] = board.state();
	return true;
}

function canNotAttack(board: Board, player: Player): boolean {
	return true;
}

export function verifyIntegrityBoard(board: Board): Result<Case[][], StrategoError> {
	let state: Case[][] = board.state();

	if(state.length !== 10 || state[0].length !== 10) {
		return new Err(new InitGameError("Board is not official, GO OUT OF THERE !!"));			
	}

	if (!checkEmptyLakes(state)) {
		return new Err(new InitGameError("You can not place pieces in lakes, please check again"));
	}

	return new Ok(state);
}

function checkEmptyLakes(cases: Case[][]): boolean {
	return checkLake(cases[2][4]) && 
		checkLake(cases[3][4]) && 
		checkLake(cases[2][5]) && 
		checkLake(cases[3][5]) && 
		checkLake(cases[6][4]) && 
		checkLake(cases[7][4]) && 
		checkLake(cases[6][5]) && 
		checkLake(cases[7][5]);
}

function checkLake(c: Case): boolean {
	return c.content.rank === PieceType.Null;
}
