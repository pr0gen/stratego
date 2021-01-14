import React, {Component, useEffect, useState} from "react";
import Board from "../Components/Board";
import { strategoCase } from "../Elements/StrategoCase";
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

    const initGameBoard = () => {

        const newBoard:strategoCase[] = []
        const allLetters = 'ABCDEFGHIJ'

        board.map((line, x) => {

            // @ts-ignore
            line.map((c, y) => newBoard.push({
                // @ts-ignore
                'type':c,
                'active': true,
                'isSelected': false,
                'position' : {
                    x,
                    y,
                    y_letter: allLetters[y]
                }
            }) )
        })

        return newBoard
    }

    const [gameBoard, setGameBoard] = useState(initGameBoard)

    const [pieces, setPieces] = useState([])

    useEffect(() => {
        socket.emit('get-all-cases');
        initGameBoard()
    }, []);

    socket.on('response-get-all-cases', (pieces:any) => {
        setPieces(pieces)
    })

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
