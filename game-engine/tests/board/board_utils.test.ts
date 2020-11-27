import { attack } from "../../src/board/board_utils";
import { Case, CaseState, create as createCase } from "../../src/case";
import { Color, create as createPiece, PieceType } from "../../src/piece/piece";

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

