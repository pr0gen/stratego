import React from 'react'
import Line from "../Components/Line";
import '../styles/Game.scss'
import Case from "./Case";

export default function Board({board}: any) {

    const handleClickCase = (position : any) => {

        {
            // @ts-ignore
            // eslint-disable-next-line array-callback-return
            board.map(c => {
                if (c.position.x === position.x && c.position.y === position.y) {
                    c.isSelected = true
                }
            })
        }
    }

    const getBoard = () :any => {
        return board
    }

    return (
        <div className="game-board">
            {
                // @ts-ignore
                getBoard().map(c  =>
                <Case key={c.position.x.toString() + c.position.y.toString()}
                      type={c.type}
                      position={c.position}
                      eventClick={handleClickCase}
                />)}
        </div>
    )

}
