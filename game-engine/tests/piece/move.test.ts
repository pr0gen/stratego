import { create, AvailableMoves } from '../../src/piece/move';

test('Should create Immovable', () => {
	let move = create(AvailableMoves.Immovable);
  expect(move.min).toBe(0);
  expect(move.max).toBe(0);
});

test('Should create Normal', () => {
	let move = create(AvailableMoves.Normal);
  expect(move.min).toBe(0);
  expect(move.max).toBe(1);
});

test('Should create Scout', () => {
	let move = create(AvailableMoves.Scout);
  expect(move.min).toBe(0);
  expect(move.max).toBe(10);
});



