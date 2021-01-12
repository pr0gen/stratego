import React, {Component, useEffect, useState} from "react";
import Case from "../Components/Case";
import Board from "../Components/Board";
import {Socket} from "../Utils/Socket";


function Game() {

    const socket = Socket.getSocket()

    const [board, setBoard] = useState([
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,'B',null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,'S',null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null],
        [null,null,null,null,null,null,null,null,null,null]
    ])

    const [pieces, setPieces] = useState([])

    useEffect(() => {
        socket.emit('get-all-cases');
    }, []);

    socket.on('response-get-all-cases', (pieces:any) => {
        setPieces(pieces)
    })

/*

   <h2>Liste des Pièces</h2>
            <div className="board-container">
                {pieces.map(piece => <Case type={piece}/> )}
            </div>
 */

    return (
        <div className="game">
            <h2>Plateau de jeux</h2>
            <div className="board-container">
                <Board board={board}/>
            </div>




        </div>
    )

}

export default Game;
