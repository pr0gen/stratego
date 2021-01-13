import React, {Component, useEffect, useState} from "react";
import Case from "../Components/Case";
import Board from "../Components/Board";
import {Socket} from "../Utils/Socket";


function Game() {

    type position = {
        x:number,
        y:number,
        y_letter: string,
    }

    type strategoCase = {
        type: string;
        position: position;
    }


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

    const [gameBoard, setGameBoard] = useState([])

    const initGameBoard = () => {

        const newBoard = []
        const allLetters = 'ABCDEFGHIJ'

        board.map((line, x) => {

            // @ts-ignore
            line.map((c, y) => newBoard.push({
                'type':c,
                'position' : {
                    x,
                    y,
                    y_letter: allLetters[y]
                }
            }) )

        })
    }



    const [pieces, setPieces] = useState([])

    useEffect(() => {
        socket.emit('get-all-cases');
    }, []);

    socket.on('response-get-all-cases', (pieces:any) => {
        setPieces(pieces)
    })


    return (
        <div className="game">
            <h2 className="medium-title">Plateau de jeux</h2>
            <div className="board-container">
                <Board board={board}/>
                <br/>
            </div>
        </div>
    )

}

export default Game;
