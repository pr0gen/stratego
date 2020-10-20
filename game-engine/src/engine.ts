import { Board, StrategoBoard } from './board/board';
import { CaseState, create as createCase } from './case';
import { createAllPiece } from './piece/infra/piece_utils';
import { Player } from './player/player';
import { gameIsOver } from './engine_utils';

class GameEngine {

	private board: Board;
	private players: [Player, Player];
	private turn: boolean; // Who plays

	constructor(p1: Player, p2: Player) {
		this.board = new StrategoBoard(10);
		this.players = [p1, p2];
		this.turn = true;
	}

	public launch() {
		while(!this.isOver()) {
			if(this.turn == true) {
				this.turn = false;// next is p2


			} else {
				this.turn = true;// next is p1
			}
		}
	}

	private askMove(player: Player) {
		//TODO use socket to ask player

	}


	private isOver(): boolean {
		return gameIsOver();
	}

}

