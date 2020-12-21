import {Room} from "./Room";
import {Player} from "./Player";

export class Rooms {

    rooms :Room[];

    constructor() {
        this.rooms = []
    }

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


    getRoomByCode(code: string) :Room {

        for (const room of this.rooms) {
            if (room.code === code) {
                return room
            }
        }

        return null
    }

}
