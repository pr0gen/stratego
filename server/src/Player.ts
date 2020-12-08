import {PlayerState} from "./PlayerState";

export class Player {
    id : string
    state : PlayerState

    constructor(id:string, state:PlayerState) {
        this.id = id
        this.state = state
    }
}
