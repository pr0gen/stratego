import React from 'react'
import Case from "./Case";

export default function Line({line}: any) {

    const temp:string[] = line   //refacto

    return (
        <div>
            {temp.map(type => <Case type={type}/>)}
        </div>
    )

}
