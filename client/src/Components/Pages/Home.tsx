import React, {Component} from "react";
import './../../styles/Home.css'
import {Link} from "react-router-dom";

class Home extends Component {
    render() {
        return (
            <div className="home" >
                <h1>Stratego</h1>
                <ul className="home-nav">
                    <li>
                        <Link className="home-link" to="/create-game">Create a game</Link>
                    </li>
                    <li>
                        <Link className="home-link" to="/join-game">Join a game</Link>
                    </li>
                    <li>
                        <Link className="home-link" to="/rules">Rules</Link>
                    </li>
                </ul>
            </div>
        )
    }
}


export default Home;