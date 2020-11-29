import {Rooms} from "../../src/Rooms";
import {Socket} from "socket.io";


export default function leaveGame(socket: Socket, rooms : Rooms) {

    socket.on('leave-game', () => {
        rooms.deleteRoomByFirstPlayerId(socket.id)
    })

}
