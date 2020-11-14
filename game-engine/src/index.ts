import { Board, StrategoBoard } from "./board/board";
import { Case, CaseState, create, createEmpty, createUnreachable } from "./case";

let newBoard: Case[][] = [
	[create(CaseState.Full, 0, 0, {move: {min:0, max:1}, rank:17}), createEmpty(0, 1)],
	[createEmpty(1, 0),create(CaseState.Full, 0, 0, {move: {min:0, max:1}, rank:4})]
];

let board: Board = new StrategoBoard(newBoard);

console.log(board.display());
