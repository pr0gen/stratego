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

export default function App() {
    return (
        <Router>
            <div>
                <nav>
                    <ul>
                        <li>
                            <Link to="/">Home</Link>
                        </li>
                        <li>
                            <Link to="/create-game">Create Game</Link>
                        </li>
                        <li>
                            <Link to="/join-game">Join Game</Link>
                        </li>
                        <li>
                            <Link to="/rules">Rules</Link>
                        </li>
                    </ul>
                </nav>

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