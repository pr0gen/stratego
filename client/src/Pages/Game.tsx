import React, {useEffect, useState} from "react";
import Board from "../Components/Board";
import {Socket} from "../Utils/Socket";
import getDefaultBoard from "../Utils/getDefaultBoard";
import getStrategoBoard from "../Utils/getStrategoBoard";


function Game() {

    const [board, setBoard] = useState(getDefaultBoard())
    const [playerWin, setPlayerWin] = useState('')
    // @ts-ignore
    const [gameBoard, setGameBoard] = useState(getStrategoBoard(board))
    const [pieces, setPieces] = useState([])
    const [key, setKey] = useState(Date.now)
    const updateBoard = (board: any, canMove = true) => {
        console.log(canMove)
        console.log(board)
        if(!canMove) {
            console.log("je disabled les cases")
            board.forEach((piece:any) => piece.active = false)
        }
        setGameBoard(board)
        setKey(Date.now)
    }


    // Socket
    const socket = Socket.getSocket()

    useEffect(() => {
        socket.emit('get-all-cases');

        socket.on('response-get-all-cases', (board: any, canMove:boolean) => {
            setPieces([])

            if(board.message !== 'None') {
                setPlayerWin(board.message)
            }

            console.log("can move =>" + canMove)
            updateBoard(getStrategoBoard(board.cases), canMove)
        })
        
        return () => {
            socket.off('response-get-all-cases');
        }

    }, []);

    const refreshBoard = () => {
        socket.emit('get-all-cases');
    }


    return (
        <div className="game">
            <h2 className="medium-title">Plateau de jeux</h2>
            <div className={playerWin ? 'modal is-active' : 'modal'}>
                <div className="modal-background"></div>
                <div className="modal-content">
                    <article className="message is-dark">
                        <div className="message-body has-text-centered">

                            <h1 className="is-size-3 mb-5">Partie terminé</h1>
                            <p className="is-size-5	mb-3"> Le gagnant de la partie est le joueur : {playerWin} </p>
                        </div>
                    </article>
                </div>
                <button className="modal-close is-large" aria-label="close"></button>
            </div>
            <div className="board-container">
                <Board
                    key={key}
                    board={gameBoard}
                    setGameBoard={updateBoard}
                    refreshBoard={refreshBoard}
                />
                <br/>
            </div>
        </div>
    )

}

export default Game;
