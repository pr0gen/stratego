import { Err, Ok, Result } from 'ts-results';
import { InitGameError, StrategoError } from './error/error';
import { Color } from './piece/piece';
import { Case, CaseState } from './case';

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
    if (!checkPlayerHasPieceInTheForRowsOfHisColor(state.slice(0, 4), Color.Blue) &&
        !checkPlayerHasPieceInTheForRowsOfHisColor(state.slice(5, 9), Color.Red)) {
        return new Err(new InitGameError("Players must place theirs pieces in the right place !"));
    }

    return new Ok(state);
}

function checkPlayerHasPieceInTheForRowsOfHisColor(cases: Case[][], color: Color): boolean {
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

