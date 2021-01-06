import React from "react";
import CreateGame from './Pages/CreateGame';
import Home from './Pages/Home';
import JoinGame from './Pages/JoinGame';
import Rules from './Pages/Rules';
import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link
} from "react-router-dom";
import './../styles/App.css'

export default function App() {
    return (
        <Router>
            <div>
                <Route path={['/create-game' , '/join-game' , '/rules']}>
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
                    <Route path="/create-game" component={CreateGame}>
                        <CreateGame />
                    </Route>
                    <Route path="/join-game">
                        <JoinGame />
                    </Route>
                    <Route path="/rules">
                        <Rules />
                    </Route>
                </Switch>
            </div>
        </Router>
    );
}