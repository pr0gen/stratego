import React from "react";
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
                    <Route path="/">
                        <Home />
                    </Route>
                    <Route path="/create-game">
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

function Home() {
    return <h2>Home</h2>;
}

function CreateGame() {
    return <h2>CreateGame</h2>;
}

function JoinGame() {
    return <h2>JoinGame</h2>;
}

function Rules() {
    return <h2>Rules</h2>;
}