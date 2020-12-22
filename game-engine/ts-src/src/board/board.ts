import { Ok, Err, Result } from 'ts-results';
import { Case, CaseState, createEmpty, create as createCase, display as displayCase, createUnreachable } from '../case';
import { InitGameError, MoveError, StrategoError } from '../error/error';
import { attack, checkPieceMove } from './board_utils';
import { verifyBoardIntegrity } from '../engine_utils';

export interface Coordinate {
    x: number,
    y: number
}

export interface Board {

    board: Case[][];

    state(): Case[][];

    move(c: Case, to: Coordinate): Result<Case[], StrategoError>;

    state(): Case[][];

    display(): string;

}

export class StrategoBoard implements Board {

    board: Case[][];

    public static createStategoBoard(cases: Case[][]): Result<Board, StrategoError> {
        const res = verifyBoardIntegrity(cases);
        if (res.err) {
            return new Err(new InitGameError("You board is illegal"));
        }
        return new Ok(new StrategoBoard(res.val));
    }

    public static createEmptyStrategoBoard(): Board {
        const size = 10;
        const board: Case[][] = new Array(size);
        for (var i: number = 0; i < size; i++) {
            board[i] = new Array(size);
            for (var j: number = 0; j < size; j++) {
                board[i][j] = createEmpty(i, j);
            }
        }

        board[4][2] = createUnreachable(4, 2);
        board[4][3] = createUnreachable(4, 3);
        board[5][2] = createUnreachable(5, 2);
        board[5][3] = createUnreachable(5, 3);
        board[4][6] = createUnreachable(4, 6);
        board[4][7] = createUnreachable(4, 7);
        board[5][6] = createUnreachable(5, 6);
        board[5][7] = createUnreachable(7, 5);

        return new StrategoBoard(board);
    }

    constructor(board: Case[][]) {
        this.board = board;
    }

    state(): Case[][] {
        return this.board;
    }

    move(c: Case, to: Coordinate): Result<Case[], StrategoError> {
        if (!checkPieceMove(c, to)) {
            return new Err(new MoveError("Your piece can not go there", c, to));
        }
        const piece = c.content;
        const aimCase = this.board[to.x][to.y];
        switch (aimCase.state) {
            case CaseState.Empty: {
                const newCase = createCase(CaseState.Full, to.x, to.y, piece);
                this.board[to.x][to.y] = newCase;
                this.board[c.x][c.y] = createEmpty(c.x, c.y);
                return Ok([newCase]);
            }
            case CaseState.Full: {
                const result = attack(createCase(CaseState.Full, c.x, c.y, piece), aimCase);
                this.board[c.x][c.y] = result[0];
                this.board[to.x][to.y] = result[1];
                return Ok([result[0], result[1]]);
            }
            case CaseState.Unreachable: {
                return Err(new MoveError("Case is Unreachable", c, to));
            }
            default: {
                return Err(new MoveError("How can you possibly arrive there, you cheater ! ", c, to));
            }
        }
    }

    display(): string {
        let display: string = " | ";
        for (let i = 0; i < this.board.length; i++) {
            display += "  " + i + "   |  ";
        }
        display += "\n";
        for (let i = 0; i < this.board.length; i++) {
            let row = this.board[i];
            display += i + "| " + row.map(c => displayCase(c) + " | ") + "\n";
        }
        return display;
    }
}

