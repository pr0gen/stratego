import React, {useEffect, useState} from 'react'
import '../styles/Game.scss'
import Case from "./Case";
import {Socket} from "../Utils/Socket";

export default function Board({board, setGameBoard, refreshBoard}: any) {

    const socket = Socket.getSocket()

    useEffect(() => {

        socket.on('response-get-available-moves', (moves: object[]) => {

            board.forEach( (c:any) => c.canBeSelected = false)

            moves.forEach((move:any) =>
                board.forEach( (c:any) => {
                    if (c.position.x === move.to_x && c.position.y_letter === move.to_y) {
                        c.canBeSelected = true
                    }
                })
            )
            setGameBoard(board)

        })

        socket.on('response-move-piece', (moves: object[]) => {
            refreshBoard()
        })

        return () => {
            socket.off('response-get-available-moves');
            socket.off('response-move-piece');
        }

    }, []);



    const handleClickCase = (position: any) => {
        const piece = board.find((c:any) => c.position.x === position.x && c.position.y === position.y)

        if (piece.type !== null && piece.type !== '?')  {
            console.log("je clique sur une piece")
            socket.emit('get-available-moves', piece.position);
            board.forEach((c:any) => c.isSelected = (c.position.x === position.x && c.position.y === position.y))
            setGameBoard(board)

        } else {
            console.log("je ne clique pas sur une piece")
            if (piece.canBeSelected) {
                console.log("je déplace le pion a la position =>", position)
                board.forEach((c:any) => {
                    if(c.isSelected === true) {
                        socket.emit('move-piece', c.position, piece.position );
                    }

                })

            }
        }


    }

    return (
        <div className="game-board">
            {
                board.map((c:any) =>
                    <Case key={c.position.x.toString() + c.position.y.toString()}
                          type={c.type}
                          position={c.position}
                          eventClick={handleClickCase}
                          canBeSelected={c.canBeSelected}
                          isSelected={c.isSelected}
                          active={c.active}
                    />)
            }
        </div>
    )

}
