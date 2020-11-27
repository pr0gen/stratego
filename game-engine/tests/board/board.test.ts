import * as board from '../../src/board/board';
import { PieceType, create as createPiece, Color } from '../../src/piece/piece';
import { create as createCase, CaseState, Case, createEmpty, createUnreachable } from '../../src/case';

test('Should build a StrategoBoard', () => {
    let strategoBoard = board.StrategoBoard.createEmptyStrategoBoard();
    let state = strategoBoard.state();

    expect(state.length).toBe(10);
    expect(state[0].length).toBe(10);
});

test('Should place a piece in board', () => {
    let bomb = createPiece(PieceType.Bomb, Color.Blue);
    let newBoard: Case[][] = [
        [createCase(CaseState.Full, 0, 0, bomb)]
    ];

    let strategoBoard = new board.StrategoBoard(newBoard);
    let state = strategoBoard.state();

    let actualCase = state[0][0];
    let content = actualCase.content;

    expect(content.rank).toBe(PieceType.Bomb);
    expect(content.move).toEqual({ min: 0, max: 0 });
    expect(content.color).toBe(Color.Blue);

    expect(actualCase.state).toBe(1);
    expect(actualCase.x).toBe(0);
    expect(actualCase.y).toBe(0);
});

test('Should move piece', () => {
    let general = createPiece(PieceType.General, Color.Blue);
    let newBoard: Case[][] = [
        [createCase(CaseState.Full, 0, 0, general), createEmpty(0, 1)],
        [createEmpty(1, 0), createEmpty(1, 1)],
    ];

    let strategoBoard = new board.StrategoBoard(newBoard);

    let res = strategoBoard.move(createCase(CaseState.Full, 0, 0, general), { x: 1, y: 1 });
    if (res.ok) {
        let p = res.val[0];

        expect(p.content.rank).toBe(PieceType.General);

        let state = strategoBoard.state();

        let actualCase = state[1][1];
        let content = actualCase.content;

        expect(content.rank).toBe(PieceType.General);
        expect(content.move).toEqual({ min: 0, max: 1 });
        expect(content.color).toBe(Color.Blue);

        expect(actualCase.state).toBe(1);
        expect(actualCase.x).toBe(1);
        expect(actualCase.y).toBe(1);
    } else {
        //If it fails we show it, should not happen
        expect(1).toBe(0);
    }
});

test('Should not move piece cause unreachable', () => {
    let piece = createPiece(PieceType.Sergeant, Color.Blue);
    let newBoard: Case[][] = [
        [createEmpty(0, 0), createEmpty(0, 1)],
        [createEmpty(1, 0), createUnreachable(1, 1)]
    ];
    let strategoBoard = new board.StrategoBoard(newBoard);

    let res = strategoBoard.move(createCase(CaseState.Full, 1, 1, piece), { x: 1, y: 1 });

    expect(res.err).toBe(true);
});

test('Should move and capture', () => {
    let sergeant = createPiece(PieceType.Sergeant, Color.Blue);
    let lieutenant = createPiece(PieceType.Lieutenant, Color.Red);
    let newBoard: Case[][] = [
        [createCase(CaseState.Full, 0, 0, lieutenant), createCase(CaseState.Full, 0, 1, sergeant)],
    ];

    let strategoBoard = new board.StrategoBoard(newBoard);
    let res = strategoBoard.move(createCase(CaseState.Full, 0, 0, lieutenant), { x: 0, y: 1 });

    if (res.ok) {
        let cases = res.val;
        let previousCase = cases[0];

        expect(previousCase.state).toBe(CaseState.Empty);

        let actualCase = cases[1];
        expect(actualCase.state).toBe(CaseState.Full);
        let piece = actualCase.content;
        expect(piece.color).toBe(Color.Red);
        expect(piece.rank).toBe(PieceType.Lieutenant);

    } else {
        //If it fails we show it, should not happen
        console.log(res.val);
        expect(1).toBe(0);
    }
});
