
// TO DO : clean code

export const boardParser = (chaine:string) => {

    let lines :string[]= chaine.split('\n')
    lines.shift()

    const newLines :string[][] = []
    for (let i = 0; i < lines.length; i++) {
        let temp = lines[i].split('|')
        temp.shift
        newLines.push(temp)
    }

    let board :(string|null)[][]= []

    newLines.forEach(line => {
        line.slice(-1,1)
        let temp :(string|null)[]= []
        line.forEach(piece => {

            piece = piece.slice(0,-2).trim()

            let newPiece :string|null
            switch(piece) {

                case '':
                    newPiece = null
                    break

                case '----':
                    newPiece = '?'
                    break

                case 'XXXX':
                    newPiece = 'water'
                    break

                default:
                    newPiece = piece
                    break
            }

            temp.push(newPiece)

        })
        temp.shift()
        temp.splice(-1,1)
        board.push(temp)
    })
    board.splice(-1,1)
    return board
}
