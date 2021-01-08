import React, {Component, useEffect, useState} from "react";
import Case from "../Components/Case";
import Line from "../Components/Line";
import {Socket} from "../Utils/Socket";


function Game() {

    console.log(Socket.getSocket)
    const [board, setBoard] = useState([
        ['1', '1', '1', '1'],
        ['2', '1', '1', '1', '1'],
        ['3'],
        ['4'],
        ['5'],
        ['6'],
        ['7'],
        ['8'],
        ['9'],
        ['10'],
    ])

    const update = () => {
        setBoard([
            ['2', '1', '1', '1', '1'],
            ['2', '2', '1', '1', '1'],
            ['2', '1', '3', '1', '1'],
            ['2', '1', '4', '1', '1'],
            ['2', '1', '1', '5', '1'],
            ['2', '1', '1', '1', '6'],
            ['2', '1', '1', '7', '1'],
            ['2', '1', '8', '1', '1'],
            ['2', '9', '1', '1', '1'],
            ['2', '3', '4', '1', '1'],
        ])
    }


    console.log(board)
    //const boardHTML : string = "<Board />";

    return (
        <div className="game">
            <h2>Plateau</h2>
            <div className="board-container">
                {board.map(line => <Line line={line}/> )}
            </div>
            <button onClick={update}> Update </button>
        </div>
    )

}


export default Game;
