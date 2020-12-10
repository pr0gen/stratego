import * as Engine from '../src/engine_utils';
import { Board, StrategoBoard } from '../src/board/board';
import { Case, CaseState, createEmpty, createUnreachable, create as createCase } from '../src/case';
import { InitGameError } from '../src/error/error';
import { PieceType, Color, create as createPiece } from '../src/piece/piece';


test('Game should be over - because blue has looses Flag', () => {
    let cases = emptyBoard()

    //Red
    cases[0][0] = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Flag, Color.Red));
    cases[0][1] = createCase(CaseState.Full, 0, 1, createPiece(PieceType.General, Color.Red));

    //Blue
    cases[9][1] = createCase(CaseState.Full, 9, 1, createPiece(PieceType.General, Color.Blue));

    let board = new StrategoBoard(cases);

    let res = Engine.gameIsOver(board);
    expect(res.ok).toBe(true);
    expect(res.val).toBe(Color.Red);
});

test('Game should be over - because blue has looses moveable pieces', () => {
    let cases = emptyBoard()

    //Red
    cases[0][0] = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Flag, Color.Red));
    cases[0][1] = createCase(CaseState.Full, 0, 1, createPiece(PieceType.General, Color.Red));

    //Blue
    cases[9][0] = createCase(CaseState.Full, 9, 0, createPiece(PieceType.Flag, Color.Blue));

    let board = new StrategoBoard(cases);

    let res = Engine.gameIsOver(board);
    expect(res.ok).toBe(true);
    expect(res.val).toBe(Color.Red);
});

test('Game should not be over', () => {
    let cases = emptyBoard()

    //Red
    cases[0][0] = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Flag, Color.Red));
    cases[0][1] = createCase(CaseState.Full, 0, 1, createPiece(PieceType.General, Color.Red));

    //Blue
    cases[9][0] = createCase(CaseState.Full, 9, 0, createPiece(PieceType.Flag, Color.Blue));
    cases[9][1] = createCase(CaseState.Full, 9, 1, createPiece(PieceType.General, Color.Blue));

    let board = new StrategoBoard(cases);

    let res = Engine.gameIsOver(board);
    expect(res.err).toBe(true);
});

function emptyBoard(): Case[][] {
    let size = 10;
    let board = new Array(size);
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

    return board;
}

test('Should not verify board integrity cause to small', () => {
    let newBoard: Case[][] = [
        [createEmpty(0, 0), createEmpty(0, 1)],
        [createEmpty(1, 0), createUnreachable(1, 1)]
    ];
    let res = Engine.verifyBoardIntegrity(newBoard);

    expect(res.err).toBe(true);
    expect(res.val).toStrictEqual(new InitGameError("Board is not official, GO OUT OF THERE !!"));
});

test('Should not verify board integrity cause lakes are empty', () => {
    let newBoard: Case[][] = createStrategoBoard();

    newBoard[4][2] = createCase(CaseState.Full, 4, 2, { move: { min: 0, max: 1 }, rank: 1, color: Color.Blue });

    let res = Engine.verifyBoardIntegrity(newBoard);

    expect(res.err).toBe(true);
    expect(res.val).toStrictEqual(new InitGameError("You can not place pieces in lakes, please check again"));
});

test('Should check only 2 rows in middle are empty', () => {
    let newBoard: Case[][] = new Array(10);

    for (let i = 0; i < 10; i++) {
        newBoard[i] = new Array(10);
        for (let j = 0; j < 10; j++) {
            newBoard[i][j] = createCase(
                CaseState.Full,
                i,
                j,
                { move: { min: 0, max: 1 }, rank: 1, color: Color.Blue }
            );
        }
    }

    newBoard[4][2] = createUnreachable(4, 2);
    newBoard[4][3] = createUnreachable(4, 3);
    newBoard[5][2] = createUnreachable(5, 2);
    newBoard[5][3] = createUnreachable(5, 3);
    newBoard[4][6] = createUnreachable(4, 6);
    newBoard[4][7] = createUnreachable(4, 7);
    newBoard[5][6] = createUnreachable(5, 6);
    newBoard[5][7] = createUnreachable(7, 5);

    //console.log("2 rows\n" + board.display());
    let res = Engine.verifyBoardIntegrity(newBoard);

    expect(res.err).toBe(true);
    expect(res.val).toStrictEqual(new InitGameError("The 2 rows in the middle must be empty, :("));
});

test('Should check players have placed theirs pieces in the 4 rows', () => {
    let newBoard: Case[][] = new Array(10);

    for (let i = 0; i < 10; i++) {
        newBoard[i] = new Array(10);
        for (let j = 0; j < 10; j++) {
            if (i < 4) {
                newBoard[i][j] = createCase(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 1, color: Color.Blue }
                );
            } else if (i > 5) {
                newBoard[i][j] = createCase(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 1, color: Color.Red }
                );
            } else {
                newBoard[i][j] = createEmpty(i, j);
            }
        }
    }

    // Placing a piece to the bad place
    newBoard[0][4] = createCase(CaseState.Full, 0, 4, { move: { min: 0, max: 1 }, rank: 1, color: Color.Red });

    newBoard[4][2] = createUnreachable(4, 2);
    newBoard[4][3] = createUnreachable(4, 3);
    newBoard[5][2] = createUnreachable(5, 2);
    newBoard[5][3] = createUnreachable(5, 3);
    newBoard[4][6] = createUnreachable(4, 6);
    newBoard[4][7] = createUnreachable(4, 7);
    newBoard[5][6] = createUnreachable(5, 6);
    newBoard[5][7] = createUnreachable(7, 5);

    //console.log("4 rows\n" + board.display());
    let res = Engine.verifyBoardIntegrity(newBoard);


    expect(res.err).toBe(true);
    expect(res.val).toStrictEqual(new InitGameError("Players must place theirs pieces in the right place !"));

});

test('Should check players have the right pieces', () => {
    let cases = createStrategoBoard();
    let board = new StrategoBoard(cases);
    //console.log(board.display());
    let res = Engine.verifyBoardIntegrity(cases);


    expect(res.err).toBe(true);
    expect(res.val).toStrictEqual(new InitGameError("You need to start with the right pieces"));
});

//test('Should verify board integrity', () => {
//let res = Engine.verifyBoardIntegrity(createStrategoBoard());

//expect(res.ok).toBe(true);
//});

//Bad board
function createStrategoBoard(): Case[][] {
    let newBoard: Case[][] = new Array(10);

    for (let i = 0; i < 10; i++) {
        newBoard[i] = new Array(10);
        for (let j = 0; j < 10; j++) {
            if (i < 4) {
                newBoard[i][j] = createCase(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 1, color: Color.Blue }
                );
            } else if (i > 5) {
                newBoard[i][j] = createCase(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 1, color: Color.Red }
                );
            } else {
                newBoard[i][j] = createEmpty(i, j);
            }
        }
    }

    newBoard[4][2] = createUnreachable(4, 2);
    newBoard[4][3] = createUnreachable(4, 3);
    newBoard[5][2] = createUnreachable(5, 2);
    newBoard[5][3] = createUnreachable(5, 3);
    newBoard[4][6] = createUnreachable(4, 6);
    newBoard[4][7] = createUnreachable(4, 7);
    newBoard[5][6] = createUnreachable(5, 6);
    newBoard[5][7] = createUnreachable(7, 5);

    return newBoard;
}

