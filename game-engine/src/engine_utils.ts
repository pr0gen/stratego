import { Board } from './board/board';
import { Case } from './case';

export function gameIsOver(board: Board): boolean {
	return canNotAttack(board) && canNotMove(board);
}

function canNotMove(board: Board): boolean {
	let actualBoard: Case[][] = board.state();
	return true;
}

function canNotAttack(board: Board): boolean {
	return true;
}

