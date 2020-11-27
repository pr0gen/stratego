import { AvailableMoves, create as createMove } from './piece/move';
import { Color, Piece, PieceType, display as displayPiece } from './piece/piece';

export interface Case {
    state: CaseState,
    x: number,
    y: number,
    content: Piece
}

export enum CaseState {
    Unreachable = -1,
    Empty = 0,
    Full = 1
}

export function display(c: Case): string {
    const content = c.content;
    switch(c.state) {
      case CaseState.Empty: {
        return "     ";
      }
      case CaseState.Unreachable: {
        return "XXXXX";
      }
      case CaseState.Full: {
        return displayPiece(content);
      }
    }
}

export function createUnreachable(x: number, y: number): Case {
    return create(
        CaseState.Unreachable,
        x, y,
        { move: createMove(AvailableMoves.Immovable), rank: PieceType.Null, color: Color.None }
    );
}

export function createEmpty(x: number, y: number): Case {
    return create(
        CaseState.Empty,
        x, y,
        { move: createMove(AvailableMoves.Immovable), rank: PieceType.Null, color: Color.None }
    );
}

export function create(state: CaseState,
    x: number,
    y: number,
    content: Piece): Case {

    return { state, x, y, content };
}
