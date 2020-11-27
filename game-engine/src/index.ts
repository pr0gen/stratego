import { Board, StrategoBoard } from "./board/board";
import { createStrategoBoard } from "./board/board_utils";
import { Case, CaseState, create, createEmpty, createUnreachable } from "./case";
import { Color } from "./piece/piece";


const board: Board = createStrategoBoard();
console.log(board.display());
const state = board.state();
board.move(state[][]);

console.log(board.display());

