import {PlayerState} from "./PlayerState";

export class Player {
    id : string
    state : PlayerState

    constructor() {
        this.id = ''
        this.state = PlayerState.Null
    }

}
