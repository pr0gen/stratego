import { Board } from './board/board';
import { createPiecesForNormalGame } from './piece/infra/piece_utils';
import { Player } from './player/player';
import * as engine from './engine_utils';

export class GameEngine {

    private board: Board;
    private players: [Player, Player];
    private turn: boolean; // Who plays true = p1

    constructor(p1: Player, p2: Player, board: Board) {
        this.board = board;
        this.players = [p1, p2];
        this.turn = true;
    }

    public launch(): void {
        while (!this.isOver()) {

        }
    }

    private isOver(): boolean {
        return true;
    }

    public getBoard(): Board {
        return this.board;
    }
}


