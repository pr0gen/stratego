import React from 'react'
import Line from "../Components/Line";
import '../styles/Game.scss'
import Case from "./Case";
import {StrategoBoard} from "../Elements/StrategoBoard";
import getDefaultBoard from "../Utils/getDefaultBoard";

export default function Board({board, setGameBoard}: any) {

    const handleClickCase = (position: any) => {
        console.log(position)

        setGameBoard(new StrategoBoard(getDefaultBoard()))
    }

    return (
        <div className="game-board">
            {
                // @ts-ignore
                board.get().map(c =>
                    <Case key={c.position.x.toString() + c.position.y.toString()}
                          type={c.type}
                          position={c.position}
                          eventClick={handleClickCase}
                          isSelected={c.isSelected}
                    />)}
        </div>
    )

}
