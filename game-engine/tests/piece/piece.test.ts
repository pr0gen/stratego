import { Color, create, PieceType } from '../../src/piece/piece';

test('Should create Null', () => {
    let piece = create(PieceType.Null, Color.None);
    expect(piece.rank).toBe(-10);
});

test('Should create Bomb', () => {
    let bomb = create(PieceType.Bomb, Color.None);
    expect(bomb.rank).toBe(-2);
});

test('Should create Marshal', () => {
    let marshal = create(PieceType.Marshal, Color.None);
    expect(marshal.rank).toBe(10);
});

test('Should create General', () => {
    let piece = create(PieceType.General, Color.None);
    expect(piece.rank).toBe(9);
});

test('Should create Colonel', () => {
    let piece = create(PieceType.Colonel, Color.None);
    expect(piece.rank).toBe(8);
});

test('Should create Major', () => {
    let piece = create(PieceType.Major, Color.None);
    expect(piece.rank).toBe(7);
});

test('Should create Captain', () => {
    let piece = create(PieceType.Captain, Color.None);
    expect(piece.rank).toBe(6);
});

test('Should create Lieutenant', () => {
    let piece = create(PieceType.Lieutenant, Color.None);
    expect(piece.rank).toBe(5);
});

test('Should create Sergeant', () => {
    let piece = create(PieceType.Sergeant, Color.None);
    expect(piece.rank).toBe(4);
});

test('Should create Miner', () => {
    let piece = create(PieceType.Miner, Color.None);
    expect(piece.rank).toBe(3);
});

test('Should create Scout', () => {
    let piece = create(PieceType.Scout, Color.None);
    expect(piece.rank).toBe(2);
});

test('Should create Spy', () => {
    let piece = create(PieceType.Spy, Color.None);
    expect(piece.rank).toBe(1);
});

test('Should create Flag', () => {
    let piece = create(PieceType.Flag, Color.None);
    expect(piece.rank).toBe(-1);
});

