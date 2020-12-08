import {Player} from "./Player";
import {PlayerState} from "./PlayerState";

export class Players {

    players :Player[];

    constructor() {
        this.players = []
    }

    addPlayer(socketId:string, playerState : PlayerState ) {
        let player = new Player(socketId, playerState)
        this.players.push(player)
    }

}
