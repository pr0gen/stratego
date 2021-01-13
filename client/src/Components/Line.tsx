import React from 'react'
import Case from "./Case";
import '../styles/Game.scss'

export default function Line({line}: any) {

    const temp:string[] = line   //refacto

    return (
        <div className="">l
            <div className="columns line">
                {temp.map(type => <Case type={type}/>)}
            </div>
        </div>
    )

}
