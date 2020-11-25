import {Room} from "./Room";
import {Player} from "./Player";

export class Rooms {

    rooms :Room[] = [];

    constructor() {}

    addRoom(room: Room) {
        this.rooms.push(room)
    }

}
