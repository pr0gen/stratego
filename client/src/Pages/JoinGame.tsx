import React, {Component, useState} from "react";
import {Redirect} from "react-router-dom"
import {Socket} from "../Utils/Socket";


function JoinGame() {

    const socket = Socket.getSocket()
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

            <h2 className="medium-title">Join Game</h2>

            <div className="join-code has-text-centered">

                <div className="control">
                    <input className="input is-large is-rounded" type="text" placeholder="Game Code" value={code} onChange={handleChangeCode}/>
                </div>

                <button className="button is-primary mt-3" onClick={checkCode}> Code verification </button>
            </div>


        </div>
    )


}


export default JoinGame;
