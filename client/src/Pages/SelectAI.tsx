import React, {useEffect, useState } from "react";
import {Redirect} from "react-router-dom"
import {Socket} from "../Utils/Socket";

function SelectAI() {

    const socket = Socket.getSocket()
    const [canRedirect, setRedirect] = useState(false);

    const startGame = (AiName: string) => {
        socket.emit('play-with-ai', AiName)
    }

    useEffect(() => {

        socket.on('response-play-with-ai', (iaName:string) => {
            console.log(iaName)
            setRedirect(true)
        })

        return () => socket.off('response-play-with-ai');

    }, []);


    return (
        <div className="select-ai">
            {canRedirect ? <Redirect to={"/game"} /> : null}

            <h1 className="medium-title">Select AI</h1>
            <div className="has-text-centered">
                <ul className="home-nav">
                    <li>
                        <button className="button is-primary is-large mb-5 home-link"
                                onClick={() => startGame("random")}>
                            Random
                        </button>
                    </li>
                    <li>
                        <button className="button is-primary is-large mb-5 home-link"
                                onClick={() => startGame("monte-carlo")}>
                            Monte Carlo
                        </button>
                    </li>
                </ul>
            </div>
        </div>
    )
}


export default SelectAI;
