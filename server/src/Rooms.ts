import {Room} from "./Room";
import {Player} from "./Player";

export class Rooms {

    rooms :Room[] = [];

    constructor() {}

    addRoom(room: Room) {
        this.rooms.push(room)
    }

    deleteRoomByFirstPlayerId(playerId : string) {

        let newRooms = new Rooms()

        for (const room of this.rooms) {
            if (room.firstPlayerId != playerId) {
                newRooms.addRoom(room)
            }
        }

        this.rooms = newRooms.rooms

    }

}
