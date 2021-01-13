import React, {useState} from 'react'
import '../styles/Game.scss'
export default function Case({type,position, eventClick} : any) {

    const [active, setActive] = useState(false)
    const handleClick = () => {
        setActive(!active )
        eventClick(position)
    }




    return (
        <div className={active ? 'active game-box' : 'game-box'} onClick={handleClick} >
            {/*<span>  {type ? `${type}` : '_'}  </span>*/}
        </div>

    )

}
