import { attack, checkPieceMove } from "../../src/board/board_utils";
import { Case, CaseState, create as createCase } from "../../src/case";
import { Color, create as createPiece, Piece, PieceType } from "../../src/piece/piece";

test('Attacker should win', () => {
    const attacker: Case = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Colonel, Color.Blue));
    const defenser: Case = createCase(CaseState.Full, 0, 1, createPiece(PieceType.Lieutenant, Color.Red));

    const res = attack(attacker, defenser);

    expect(res[0].state).toBe(CaseState.Empty);

    expect(res[1].state).toBe(CaseState.Full);
    expect(res[1].content.color).toBe(Color.Blue);
});

test('Defenser should win', () => {
    const attacker: Case = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Lieutenant, Color.Blue));
    const defenser: Case = createCase(CaseState.Full, 0, 1, createPiece(PieceType.Colonel, Color.Red));

    const res = attack(attacker, defenser);

    expect(res[1].state).toBe(CaseState.Empty);

    expect(res[0].state).toBe(CaseState.Full);
    expect(res[0].content.color).toBe(Color.Red);
});

test('Both should loose', () => {
    const attacker: Case = createCase(CaseState.Full, 0, 0, createPiece(PieceType.Colonel, Color.Blue));
    const defenser: Case = createCase(CaseState.Full, 0, 1, createPiece(PieceType.Colonel, Color.Red));

    const res = attack(attacker, defenser);

    expect(res[0].state).toBe(CaseState.Empty);

    expect(res[1].state).toBe(CaseState.Empty);
});

test('Should enable move', () => {
    let piece: Piece = createPiece(PieceType.General, Color.Blue);
    let c: Case = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 0, y: 1 })).toBe(true);

    piece = createPiece(PieceType.Scout, Color.Blue);
    c = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 0, y: 9 })).toBe(true);

    piece = createPiece(PieceType.General, Color.Blue);
    c = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 1, y: 0 })).toBe(true);
});

test('Shouldnt enable move', () => {
    let piece: Piece = createPiece(PieceType.General, Color.Blue);
    let c: Case = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 0, y: 2 })).toBe(false);

    piece = createPiece(PieceType.Bomb, Color.Blue);
    c = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 0, y: 2 })).toBe(false);

    piece = createPiece(PieceType.General, Color.Blue);
    c = createCase(CaseState.Full, 0, 0, piece);
    expect(checkPieceMove(c, { x: 1, y: 1 })).toBe(false);


});
