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
            if (room.firstPlayer.id != playerId) {
                newRooms.addRoom(room)
            }
        }

        this.rooms = newRooms.rooms
    }


    getRoomByCode(code: string) :Room|undefined{
        return this.rooms.find(room => room.code === code)
    }

}
