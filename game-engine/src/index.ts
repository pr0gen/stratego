import { Board, StrategoBoard } from "./board/board";
import { createStrategoBoard } from "./board/board_utils";
import { Case, CaseState, create, createEmpty, createUnreachable } from "./case";
import { Color } from "./piece/piece";


const res = createStrategoBoard();

if(res.ok) {
  const board = res.val;

  console.log(board.display());

  let state = board.state();

  board.move(state[3][5], { x: 4, y: 5 });
  console.log(board.display());

  state = board.state();
  board.move(state[4][5], { x: 5, y: 5 });
  console.log(board.display());

  state = board.state();
  board.move(state[5][5], { x: 6, y: 5 });
  console.log(board.display());
}
