import { createPiecesForNormalGame } from '../../../src/piece/infra/piece_utils';
import { PieceType } from '../../../src/piece/piece';


test('Should create all pieces', () => {
	let pieces = createPiecesForNormalGame();	
	expect(pieces.length).toBe(40);

	let bombs = pieces.filter(piece => piece.rank === PieceType.Bomb);
	expect(bombs.length).toBe(6);

	let marshals = pieces.filter(piece => piece.rank === PieceType.Marshal);
	expect(marshals.length).toBe(1);

	let generals = pieces.filter(piece => piece.rank === PieceType.General);
	expect(generals.length).toBe(1);

	let colonels = pieces.filter(piece => piece.rank === PieceType.Colonel);
	expect(colonels.length).toBe(2);

	let captains = pieces.filter(piece => piece.rank === PieceType.Captain);
	expect(captains.length).toBe(4);

	let lieutenants = pieces.filter(piece => piece.rank === PieceType.Lieutenant);
	expect(lieutenants.length).toBe(4);

	let sergeants = pieces.filter(piece => piece.rank === PieceType.Sergeant);
	expect(sergeants.length).toBe(4);

	let miners = pieces.filter(piece => piece.rank === PieceType.Miner);
	expect(miners.length).toBe(5);

	let scouts = pieces.filter(piece => piece.rank === PieceType.Scout);
	expect(scouts.length).toBe(8);

	let spys = pieces.filter(piece => piece.rank === PieceType.Spy);
	expect(spys.length).toBe(1);

	let flags = pieces.filter(piece => piece.rank === PieceType.Flag);
	expect(flags.length).toBe(1);

});
