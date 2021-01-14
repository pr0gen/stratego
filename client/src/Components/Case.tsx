import React, {useState} from 'react'
import '../styles/Game.scss'
export default function Case({type,position, eventClick, isSelected} : any) {

    const handleClick = () => {
        eventClick(position)
        console.log(isSelected)
    }




    return (
        <div className={isSelected ? 'active game-box' : 'game-box'} onClick={handleClick} >
            {/*<span>  {type ? `${type}` : '_'}  </span>*/}
        </div>

    )

}
