import React from 'react'

export default function Case({type}: any) {

    return (
        <span>  {type ? `[${type}]` : '[_]'}  </span>
    )

}
