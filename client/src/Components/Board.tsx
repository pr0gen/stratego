import React from 'react'
import Line from "../Components/Line";
import '../styles/Game.scss'
import Case from "./Case";

export default function Board({board}: any) {

    const board2: any[]= board   //refacto

    console.log(board2)

    return (
        <div className="container">
            {board2.map(line => {
                const temp:string[] = line
                temp.map(temp => {
                    <Case type={temp}/>
                })
            } )}
        </div>
    )

}
