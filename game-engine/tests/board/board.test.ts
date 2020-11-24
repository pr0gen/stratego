import * as board from '../../src/board/board';
import { PieceType, create as createPiece, Color } from '../../src/piece/piece';
import { create as createCase, CaseState, Case, createEmpty, createUnreachable } from '../../src/case';

test('Should build a StrategoBoard', () => {
    let strategoBoard = board.StrategoBoard.createEmptyStrategoBoard(2);
    let state = strategoBoard.state();

    expect(state.length).toBe(2);
    expect(state[0].length).toBe(2);
});

test('Should place a piece in board', () => {
    let strategoBoard = board.StrategoBoard.createEmptyStrategoBoard(2);
    strategoBoard.placePiece(0, 0, createPiece(PieceType.Bomb, Color.Blue));

    let state = strategoBoard.state();

    let actualCase = state[0][0];
    let content = actualCase.content;

    expect(content.rank).toBe(PieceType.Bomb);
    expect(content.move).toEqual({ min: 0, max: 0 });
    expect(content.color).toBe(Color.Blue);
    expect(actualCase.state).toBe(1); expect(actualCase.x).toBe(0); expect(actualCase.y).toBe(0);
});

test('Should move piece', () => {
    let strategoBoard = board.StrategoBoard.createEmptyStrategoBoard(2);
    let piece = createPiece(PieceType.General, Color.Blue);
    strategoBoard.placePiece(0, 0, piece);

    let res = strategoBoard.move(createCase(CaseState.Full, 0, 0, piece), { x: 1, y: 1 });
    if (res.ok) {
        let p = res.val;
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

