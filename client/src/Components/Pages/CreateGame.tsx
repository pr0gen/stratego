import React, {Component, useEffect, useState} from "react";
import {log} from "util";
import {Redirect} from "react-router-dom";


// function CreateGame({socket}:SocketIOClient.Socket) {
function CreateGame({socket}:any) {

    const [code, setCode] = useState('');
    const [foundGame, setFoundGame] = useState(false);

    useEffect(() => {
        socket.emit('create-game');
    }, []);


    socket.on('response-create-game', (code: string) => {
        setCode(code);
        console.log(code)
    });

    socket.on('player-found', () => {
        if (!foundGame) {
            setFoundGame(true)
        }
    })

    const quitGame = () => {
        socket.emit('leave-game');
    };

    return (
        <div className="create-game">
            {foundGame ? <Redirect to={"/rules"} /> : null}
            <h2>Create Game</h2>
            <div className="content">
                <p>Votre code est le : {code}</p>
            </div>
        </div>
    )

}


export default CreateGame;
