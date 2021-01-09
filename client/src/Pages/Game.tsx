import React, {Component, useEffect, useState} from "react";
import Case from "../Components/Case";
import Line from "../Components/Line";
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

    return (
        <div className="game">
            <h2>Plateau de jeux</h2>
            <div className="board-container">
                {board.map(line => <Line line={line}/> )}
            </div>

            <h2>Liste des Pièces</h2>
            <div className="board-container">
                {pieces.map(piece => <Case type={piece}/> )}
            </div>
        </div>
    )

}

export default Game;
