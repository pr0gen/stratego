import React, {Component, useEffect, useState} from "react";


function Game({boardAPI}: any) {

    const board : string [][] = boardAPI;
    const boardHTML : string = "<Board />";


    for (let row of board) {
        for (let column of row) {

        }
    }

    return (
        <div className="game">
            <h2>Plateau</h2>
            <div className="board-container">
                {board}
            </div>
        </div>
    )

}


export default Game;
