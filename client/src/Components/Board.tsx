import React from 'react'
import Line from "../Components/Line";
import '../styles/Game.scss'
import Case from "./Case";

export default function Board({board}: any) {

    type position = {
        x:number,
        y:number,
        y_letter: string,
    }

    type strategoCase = {
        type: string;
        position: position;
    }


    const handleClickCase = (position : any) => {
        console.log(position)
    }

    const getBoard = () :strategoCase[] => {

        const newBoard:any = []
        const allLetters = 'ABCDEFGHIJ'

        // @ts-ignore
        board.map((line, x) => {

            // @ts-ignore
            line.map((c, y) => newBoard.push({
                'type':c,
                'position' : {
                    x,
                    y,
                    y_letter: allLetters[y]
                }
            }) )

        })

        return newBoard
    }

    return (
        <div className="game-board">
            {getBoard().map(c  =>
                <Case key={c.position.x.toString() + c.position.y.toString()}
                      type={c.type}
                      position={c.position}
                      eventClick={handleClickCase}
                />)}
        </div>
    )

}
