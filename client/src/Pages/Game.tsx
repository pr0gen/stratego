import React, {useEffect, useState} from "react";
import Board from "../Components/Board";
import {StrategoBoard} from "../Elements/StrategoBoard";
import {Socket} from "../Utils/Socket";
import getDefaultBoard from "../Utils/getDefaultBoard";
import getStrategoBoard from "../Utils/getStrategoBoard";


function Game() {

    // Socket
    const socket = Socket.getSocket()

    useEffect(() => {
        socket.emit('get-all-cases');
    }, []);

    socket.on('response-get-all-cases', (pieces: any) => {
        setPieces(pieces)
    })

    const [board, setBoard] = useState(getDefaultBoard())
    // @ts-ignore
    const [gameBoard, setGameBoard] = useState(getStrategoBoard(board))
    // @ts-ignore
    const [pieces, setPieces] = useState([])
    const [key, setKey] = useState(Date.now)

    const updateBoard = (board: any) => {
        setGameBoard(board)
        setKey(Date.now)
    }

    return (
        <div className="game">
            <h2 className="medium-title">Plateau de jeux</h2>
            <div className="board-container">
                <Board key={key} board={gameBoard} setGameBoard={updateBoard}/>
                <br/>
            </div>
        </div>
    )

}

export default Game;
