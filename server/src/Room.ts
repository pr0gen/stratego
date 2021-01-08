import {RoomState} from "./RoomState";
import {Player} from "./Player";

export class Room {

    firstPlayer: Player
    secondPlayer: Player
    code: string
    state: RoomState

    constructor() {
        this.firstPlayer = new Player()
        this.secondPlayer = new Player()
        this.code = ''
        this.state = RoomState.Waiting
    }

    addSecondPlayer(secondPlayerId : string) {
        this.secondPlayer.id = secondPlayerId
        this.state = RoomState.GameStarted
    }

}
