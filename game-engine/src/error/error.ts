import { Piece } from '../piece/piece';
import { Case } from '../case';

export class StrategoError implements Error {

	name: string;
	message: string;

	constructor(name: string, message: string) {
		this.name = name;
		this.message = message;
	}

}

export class MoveError extends StrategoError {

	constructor(c: Case, p: Piece) {
		super("move", "Can not move piece " + p.rank + "to {" + c.x + ", " + c.y + ")");
	}

}

export class PlacementError extends StrategoError {
	
	constructor(x: number, y: number, p: Piece) {
		super("placement", "Can not place piece " + p.rank + "to {" + x + ", " + y + ")");
	}
}

