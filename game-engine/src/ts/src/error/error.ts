import { Piece } from '../piece/piece';
import { Case } from '../case';
import { Coordinate } from '../board/board';

export class StrategoError implements Error {

    name: string;
    message: string;

    constructor(name: string, message: string) {
        this.name = name;
        this.message = message;
    }

}

export class MoveError extends StrategoError {

    constructor(c: Case, coo: Coordinate) {
        super("move", "Can not move piece " + c.content.rank + "{" + c.x + "," + c.y + "} " + "to {" + coo.x + ", " + coo.y + ")");
    }

}

export class PlacementError extends StrategoError {

    constructor(x: number, y: number, p: Piece) {
        super("placement", "Can not place piece " + p.rank + "to {" + x + ", " + y + ")");
    }
}

export class InitGameError extends StrategoError {

    constructor(message: string) {
        super("initialization", message);
    }
}


