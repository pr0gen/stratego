import {strategoCase} from "./StrategoCase";

export class StrategoBoard {

    board: strategoCase[]

    constructor(board: (any[] | (null | string)[])[]) {

        const newBoard: strategoCase[] = []

        board.map((line, x) => {

            const allLetters = 'ABCDEFGHIJ'

            // @ts-ignore
            line.map((c, y) => newBoard.push({
                // @ts-ignore
                'type':c,
                'active': true,
                'isSelected': false,
                'position' : {
                    x,
                    y,
                    y_letter: allLetters[y]
                }
            }) )
        })


        this.board = newBoard;
    }


    public get() {
        return this.board
    }

    public disableAllCase() {
        this.board.forEach(c => {
            c.isSelected = false
        })
    }
}
