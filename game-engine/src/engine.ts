import { Board, StrategoBoard } from './board/board';
import { CaseState, create as createCase } from './case';
import { createAllPiece } from './piece/infra/piece_utils';
import { Player } from './player/player';
import { gameIsOver } from './engine_utils';

let board = new StrategoBoard(10);
let pieces = createAllPiece();
let c = createCase(CaseState.Full, 0, 0, pieces[0]);

board.move(c, pieces[0]);
console.log(board.state());

class GameEngine {

	private board: Board;
	private players: Player[];

	constructor(p1: Player, p2: Player) {
		this.board = new StrategoBoard(10);
		this.players = new Array(2);
		this.players.push(p1);
		this.players.push(p2);
	}

	public launch() {
		while(!this.isOver()) {
			//TODO
		}
	}

	private isOver(): boolean {
		return gameIsOver();
	}

}


