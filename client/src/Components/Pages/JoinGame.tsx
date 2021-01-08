import React, {Component, useState} from "react";
import {Redirect} from "react-router-dom"


function JoinGame({socket}: any) {


    const [code, setCode] = useState('');
    const [foundGame, setFoundGame] = useState(false);

    function handleChangeCode(event:any) {
        setCode(event.target.value)
    }

    function checkCode() {
        console.log(code)
        socket.emit('join-game', code)
    }

    socket.on('response-join-game', (response:any) => {
        console.log(response)
        if (!foundGame) {
            setFoundGame(response.valid)
        }
    })

    return (

        <div className="join-game">
            {foundGame ? <Redirect to={"/game"} /> : null}

            <h2>Join Game</h2>
            <div className="content">
                <label>Code de la partie :</label>
                <input className="join-game-input" value={code} onChange={handleChangeCode}></input>
                <button className="join-game-btn" onClick={checkCode}>Rejoindre</button>
            </div>
        </div>
    )


}


export default JoinGame;
