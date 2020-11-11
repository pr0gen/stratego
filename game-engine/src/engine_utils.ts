import { Board } from './board/board';
import { Case } from './case';
import { Player } from './player/player';

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

