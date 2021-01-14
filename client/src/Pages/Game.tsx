import React, {useEffect, useState} from "react";
import Board from "../Components/Board";
import { StrategoBoard } from "../Elements/StrategoBoard";
import {Socket} from "../Utils/Socket";
import getDefaultBoard from "../Utils/getDefaultBoard";


function Game() {

    // Socket
    const socket = Socket.getSocket()

    useEffect(() => {
        socket.emit('get-all-cases');
    }, []);

    socket.on('response-get-all-cases', (pieces:any) => {
        setPieces(pieces)
    })

    const [board, setBoard] = useState(getDefaultBoard())
    const [gameBoard, setGameBoard] = useState(new StrategoBoard(board))
    const [pieces, setPieces] = useState([])


    return (
        <div className="game">
            <h2 className="medium-title">Plateau de jeux</h2>
            <div className="board-container">
                <Board board={gameBoard} setBoard={setGameBoard}/>
                <br/>
            </div>
        </div>
    )

}

export default Game;
