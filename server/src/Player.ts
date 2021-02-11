import {PlayerState} from "./PlayerState";

export class Player {
    id : string
    state : PlayerState
    aiName:string

    constructor() {
        this.id = ''
        this.aiName = ''
        this.state = PlayerState.Null
    }

}
