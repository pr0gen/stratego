import { Ok, Err, Result } from 'ts-results';
import { Piece, PieceType, create as createPiece, Color } from '../piece/piece';
import { Case, CaseState, createEmpty, create as createCase, display as displayCase, createUnreachable } from '../case';
import { StrategoError, MoveError, PlacementError } from '../error/error';
import { attack } from './board_utils';

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

    public constructor(board: Case[][]) {
        this.board = board;
    }

    state(): Case[][] {
        return this.board;
    }

    move(c: Case, to: Coordinate): Result<Case[], StrategoError> {
        const aimCase = this.board[to.x][to.y];
        switch (aimCase.state) {
            case CaseState.Empty: {
                return Ok([this.placePiece(to.x, to.y, c.content)]);
            }
            case CaseState.Full: {
                const result = attack(createCase(CaseState.Full, c.x, c.y, c.content), aimCase);
                return Ok([result[0], result[1]]);
            }
            case CaseState.Unreachable: {
                return Err(new MoveError(c, to));
            }
            default: {
                return Err(new MoveError(c, to));
            }
        }
    }

    private placePiece(x: number, y: number, p: Piece): Case {
        const c = createCase(CaseState.Full, x, y, p);
        this.board[x][y] = c;
        return c;
    }

    display(): string {
        return this.board
            .map(row => "|" + row.map(c => displayCase(c) + " | "))
            .join('\n');
    }
}

