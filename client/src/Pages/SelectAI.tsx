import React, {Component} from "react";

class SelectAI extends Component {

    render() {
        return (
            <div className="home">

                <h1 className="medium-title">Select AI</h1>
                <div className="has-text-centered">
                    <ul className="home-nav">
                        <li>
                            <button className="button is-primary is-large mb-5 home-link" >
                                Random
                            </button>
                        </li>
                        <li>
                            <button className="button is-primary is-large mb-5 home-link" >
                                Monte Carlo
                            </button>
                        </li>
                    </ul>
                </div>
            </div>
        )
    }
}


export default SelectAI;
