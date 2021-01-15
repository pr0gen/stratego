import {strategoCase} from "../Elements/StrategoCase";


const getStrategoBoard = (board:string[][]):strategoCase[] => {

    const allLetters = 'ABCDEFGHIJ'
    const newBoard :strategoCase[] = []

    board.map( (line,x) => {

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

    return newBoard


}

export default getStrategoBoard
