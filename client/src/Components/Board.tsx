import React from 'react'
import '../styles/Game.scss'
import Case from "./Case";

export default function Board({board, setGameBoard}: any) {

    const handleClickCase = (position: any) => {
        // @ts-ignore
        board.forEach(c => c.isSelected = (c.position.x === position.x && c.position.y === position.y))
        setGameBoard(board)
    }

    return (
        <div className="game-board">
            {
                // @ts-ignore
                board.map(c =>
                    <Case key={c.position.x.toString() + c.position.y.toString()}
                          type={c.type}
                          position={c.position}
                          eventClick={handleClickCase}
                          isSelected={c.isSelected}
                    />)
            }
        </div>
    )

}
