import React, {Component} from "react";


class JoinGame extends Component {
    render() {
        return (
            <div className="join-game" >
                <h2>Join Game</h2>
                <div className="content">
                    <label>Code de la partie :</label>
                    <input className="join-game-input"></input>
                    <button className="join-game-btn">Rejoindre</button>
                </div>
            </div>
        )
    }
}


export default JoinGame;
