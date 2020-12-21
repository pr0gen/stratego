import {RoomState} from "./RoomState";

export class Room {

    firstPlayerId: string
    secondPlayerId: string
    code: string
    state: RoomState

    constructor() {}

    addSecondPlayer(secondPlayerId : string) {
        this.secondPlayerId = secondPlayerId
        this.state = RoomState.gameStarted
    }

}
