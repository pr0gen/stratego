import { Board, StrategoBoard } from './board/board';
import { CaseState, create as createCase } from './case';
import { createPiecesForNormalGame } from './piece/infra/piece_utils';
import { Player } from './player/player';
import { gameIsOver } from './engine_utils';

class GameEngine {

	private board: Board;
	private players: [Player, Player];
	private turn: boolean; // Who plays

	constructor(p1: Player, p2: Player, board: Board) {
		this.board = board;
		this.players = [p1, p2];
		this.turn = true;
	}

	public launch(): void {
		while(!this.isOver()) {

		}
	}

	private isOver(): boolean {
		return true;
		//return gameIsOver(this.board);
	}

}

