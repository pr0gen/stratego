import React from "react";
import CreateGame from './Pages/CreateGame';
import Home from './Pages/Home';
import JoinGame from './Pages/JoinGame';
import Rules from './Pages/Rules';
import Game from './Pages/Game';
import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";
import './styles/App.css'

import io from 'socket.io-client'
import './styles/style.scss'
import { Socket } from "./Utils/Socket";

const socket: SocketIOClient.Socket = io('http://localhost:3001');


export default function App() {

    Socket.setSocket(socket)

    return (
        <Router>
            <div>
                <Route path={['/create-game' , '/join-game' , '/rules', '/game']}>
                    <nav>
                        <ul>
                            <li>
                                <Link className="nav-items" to="/">Home</Link>
                            </li>
                            <li>
                                <Link className="nav-items" to="/create-game">Create Game</Link>
                            </li>
                            <li>
                                <Link className="nav-items" to="/join-game">Join Game</Link>
                            </li>
                            <li>
                                <Link className="nav-items" to="/rules">Rules</Link>
                            </li>
                        </ul>
                    </nav>
                </Route>

                <Switch>
                    <Route exact={true} path="/">
                        <Home />
                    </Route>
                    <Route path="/create-game" component={CreateGame} />
                    <Route path="/join-game" component={JoinGame}/>
                    <Route path="/rules">
                        <Rules />
                    </Route>
                    <Route path="/game">
                        <Game />
                    </Route>
                </Switch>
            </div>
        </Router>

    );
}
