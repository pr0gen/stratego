import { create, PieceType } from '../../src/piece/piece';

test('Should create Null', () => {
	let piece = create(PieceType.Null);
  expect(piece.rank).toBe(-10);
});

test('Should create Bomb', () => {
	let bomb = create(PieceType.Bomb);
  expect(bomb.rank).toBe(-2);
});

test('Should create Marshal', () => {
	let marshal = create(PieceType.Marshal);
  expect(marshal.rank).toBe(10);
});

test('Should create General', () => {
	let piece = create(PieceType.General);
  expect(piece.rank).toBe(9);
});

test('Should create Colonel', () => {
	let piece = create(PieceType.Colonel);
  expect(piece.rank).toBe(8);
});

test('Should create Major', () => {
	let piece = create(PieceType.Major);
  expect(piece.rank).toBe(7);
});

test('Should create Captain', () => {
	let piece = create(PieceType.Captain);
  expect(piece.rank).toBe(6);
});

test('Should create Lieutenant', () => {
	let piece = create(PieceType.Lieutenant);
  expect(piece.rank).toBe(5);
});

test('Should create Sergeant', () => {
	let piece = create(PieceType.Sergeant);
  expect(piece.rank).toBe(4);
});

test('Should create Miner', () => {
	let piece = create(PieceType.Miner);
  expect(piece.rank).toBe(3);
});

test('Should create Scout', () => {
	let piece = create(PieceType.Scout);
  expect(piece.rank).toBe(2);
});

test('Should create Spy', () => {
	let piece = create(PieceType.Spy);
  expect(piece.rank).toBe(1);
});

test('Should create Flag', () => {
	let piece = create(PieceType.Flag);
  expect(piece.rank).toBe(-1);
});

