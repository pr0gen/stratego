import React, {Component} from "react";
import '../styles/Home.scss'
import {Link} from "react-router-dom";

class Home extends Component {
    render() {
        return (
            <div className="home">
                <h1 className="title">Stratego Game</h1>
                <div className="has-text-centered">
                    <ul className="home-nav">
                        <li>
                            <Link className="button is-primary is-large mb-5 home-link" to="/select-ai">
                                Play with AI
                            </Link>
                        </li>
                        <li>
                            <Link className="button is-primary is-large mb-5 home-link" to="/create-game">
                                Create game
                            </Link>
                        </li>
                        <li>
                            <Link className="button is-primary is-large mb-5 home-link" to="/join-game">
                                Join a game
                            </Link>
                        </li>
                        <li>
                            <Link className="button is-primary is-large mb-5 home-link" to="/rules">
                                Rules
                            </Link>
                        </li>
                    </ul>
                </div>
            </div>
        )
    }
}


export default Home;
