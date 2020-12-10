import { Err, Ok, Result } from 'ts-results';
import { InitGameError, StrategoError, GameNotOverError } from './error/error';
import { Color, PieceType } from './piece/piece';
import { listAllPieces } from './piece/infra/piece_utils';
import { Case, CaseState } from './case';
import { Board } from './board/board';
import { AvailableMoves, equals } from './piece/move';

export function gameIsOver(board: Board): Result<Color, StrategoError> {
    let flattenState: Case[] = ([] as Case[]).concat(...board.state());
    let blues = flattenState.filter(c => c.content.color === Color.Blue);
    let reds = flattenState.filter(c => c.content.color === Color.Red);

    //Check has Flag
    let res = blues.filter(c => c.content.rank === PieceType.Flag);
    if (0 === res.length) {
        return new Ok(Color.Red);
    }
    res = reds.filter(c => c.content.rank === PieceType.Flag);
    if (0 === res.length) {
        return new Ok(Color.Blue);
    }

    //Check moveable pieces
    res = blues.filter(c => c.content.rank !== PieceType.Flag)
        .filter(c => !equals(c.content.move, AvailableMoves.Immovable));
    if (0 === res.length) {
        return new Ok(Color.Red);
    }
    res = reds.filter(c => c.content.rank !== PieceType.Flag)
        .filter(c => !equals(c.content.move, AvailableMoves.Immovable));
    if (0 === res.length) {
        return new Ok(Color.Blue);
    }

    return new Err(new GameNotOverError());
}


//Verify the validity of the pieces e.g 1 F, 1 Spy
export function verifyBoardIntegrity(state: Case[][]): Result<Case[][], StrategoError> {
    if (!checkBoardSize(state)) {
        return new Err(new InitGameError("Board is not official, GO OUT OF THERE !!"));
    }

    if (!checkEmptyMiddle(state)) {
        if (!checkEmptyLakes(state)) {
            return new Err(new InitGameError("You can not place pieces in lakes, please check again"));
        }
        return new Err(new InitGameError("The 2 rows in the middle must be empty, :("));
    }

    // blue: 0 -> 4 || red: 5 -> 9
    if (!checkPlayerHasPieceInTheFourRowsOfHisColor(state.slice(0, 4), Color.Blue) &&
        !checkPlayerHasPieceInTheFourRowsOfHisColor(state.slice(5, 9), Color.Red)) {
        return new Err(new InitGameError("Players must place theirs pieces in the right place !"));
    }

    if (!checkPlayerHasCorrectPieces(state.slice(0, 4)) &&
        !checkPlayerHasCorrectPieces(state.slice(5, 9))) {
        return new Err(new InitGameError("You need to start with the right pieces"));
    }

    return new Ok(state);
}

function checkPlayerHasCorrectPieces(cases: Case[][]): boolean {
    let flattenState: Case[] = ([] as Case[]).concat(...cases);

    let pieceTypes = flattenState.map(c => c.content.rank)
    let pieces: Map<PieceType, number> = new Map(
        pieceTypes.map(x => [x, pieceTypes.filter(y => y === x).length])
    );

    let allPieces = listAllPieces();
    for (let [key, value] of pieces.entries()) {
        if (allPieces.get(key) !== value) {
            return false;
        }
    }
    return true;
}

function checkPlayerHasPieceInTheFourRowsOfHisColor(cases: Case[][], color: Color): boolean {
    for (let row of cases) {
        for (let c of row) {
            const piece = c.content;
            if (piece.color != color) {
                return false;
            }
        }
    }
    return true;
}


function checkEmptyMiddle(cases: Case[][]): boolean {
    let forthRow: Case[] = cases[4]
        .filter(row => !isSupposedToBeInTheMiddle(row));

    let fifthRow: Case[] = cases[5]
        .filter(row => !isSupposedToBeInTheMiddle(row));

    if (forthRow.length !== 0 || fifthRow.length !== 0) {
        return false;
    }

    return true;
}

function isSupposedToBeInTheMiddle(c: Case): boolean {
    return c.state !== CaseState.Full;
}

function checkBoardSize(cases: Case[][]): boolean {
    if (cases.length !== 10) {
        return false;
    }

    let res = cases.filter(row => !checkRowSize(row));
    if (res.length !== 0) {
        return false;
    }

    return res.length === 0;
}

function checkRowSize(row: Case[]): boolean {
    return row.length === 10;
}

function checkEmptyLakes(cases: Case[][]): boolean {
    return checkLake(cases[4][2]) &&
        checkLake(cases[4][3]) &&
        checkLake(cases[5][2]) &&
        checkLake(cases[5][3]) &&
        checkLake(cases[4][6]) &&
        checkLake(cases[4][7]) &&
        checkLake(cases[5][6]) &&
        checkLake(cases[5][7]);
}

function checkLake(c: Case): boolean {
    return c.state === CaseState.Unreachable;
}

