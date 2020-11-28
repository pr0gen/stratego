import { Result } from 'ts-results';
import { Case, CaseState, create, createEmpty, createUnreachable } from '../case';
import { StrategoError } from '../error/error';
import { Color, Piece, PieceType } from '../piece/piece';
import { Board, Coordinate, StrategoBoard } from './board';

export function attack(from: Case, to: Case): [Case, Case] {

    const attacker = from.content;
    const defenser = to.content;

    if (attacker.rank > defenser.rank) {
        return [createEmpty(from.x, from.y), create(CaseState.Full, to.x, to.y, from.content)];
    }

    if (attacker.rank === defenser.rank) {
        return [createEmpty(from.x, from.y), createEmpty(to.x, to.y)];
    }

    return [create(CaseState.Full, from.x, from.y, to.content), createEmpty(to.x, to.y)];

}

export function checkPieceMove(c: Case, to: Coordinate): boolean {
  const move = c.content.move;
  const deltaX = Math.abs(to.x - c.x);
  const deltaY = Math.abs(to.y - c.y);
  if(deltaX <= move.max && deltaY == 0 ||
     deltaX == 0 && deltaY <= move.max) {
    return true;
  }
  return false;
}



//Only for testing the global engine
export function createStrategoBoard(): Result<Board, StrategoError> {
    let newBoard: Case[][] = new Array(10);

    for (let i = 0; i < 10; i++) {
        newBoard[i] = new Array(10);
        for (let j = 0; j < 10; j++) {
            if (i < 4) {
                newBoard[i][j] = create(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 5, color: Color.Blue }
                );
            } else if (i > 5) {
                newBoard[i][j] = create(
                    CaseState.Full,
                    i,
                    j,
                    { move: { min: 0, max: 1 }, rank: 3, color: Color.Red }
                );
            } else {
                newBoard[i][j] = createEmpty(i, j);
            }
        }
    }

    newBoard[4][2] = createUnreachable(4, 2);
    newBoard[4][3] = createUnreachable(4, 3);
    newBoard[5][2] = createUnreachable(5, 2);
    newBoard[5][3] = createUnreachable(5, 3);
    newBoard[4][6] = createUnreachable(4, 6);
    newBoard[4][7] = createUnreachable(4, 7);
    newBoard[5][6] = createUnreachable(5, 6);
    newBoard[5][7] = createUnreachable(7, 5);

    return StrategoBoard.createStategoBoard(newBoard);
}
