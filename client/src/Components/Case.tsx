import React from 'react'
import '../styles/Game.scss'
import getImageUrl from "../Utils/getImageUrl";

export default function Case({type,position, eventClick, canBeSelected,isSelected} : any) {

    const handleClick = () => {
        if (type === 'water') return
        eventClick(position)
    }

    return (
        <div
            className={'game-box ' + (isSelected ? ' active border-active ' : '') + (type === 'water' ? 'water ' : '')
            + (canBeSelected ? ' border-active ' : '')  }
             onClick={handleClick}
             style={{backgroundImage: 'url(' + getImageUrl(type)+')'}}
        >
            {type === '?' ? '?' : ''}
        </div>
    )

}
