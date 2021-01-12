import React from 'react'
import '../styles/Game.scss'
import {log} from "util";
export default function Case({type}: any) {


    return (
        <div className="colum case">
            <span>  {type ? `[${type}]` : '[_]'}  </span>
        </div>

    )

}
