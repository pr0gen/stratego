import React from 'react'
import '../styles/Game.scss'
import getImageUrl from "../Utils/getImageUrl";

export default function Case({type,position, eventClick, isSelected} : any) {

    const handleClick = () => {
        if (['?', 'water'].includes(type)) return
        eventClick(position)
    }

    return (
        <div
            className={'game-box ' + (isSelected ? 'active border-active ' : '') + (type === 'water' ? 'water ' : '') }
             onClick={handleClick}
             style={{backgroundImage: 'url(' + getImageUrl(type)+')'}}
        >
            {type === '?' ? '?' : ''}
        </div>
    )

}
