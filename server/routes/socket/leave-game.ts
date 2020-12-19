import {Rooms} from "../../src/Rooms";
import socketio, {Socket} from "socket.io";


export default function leaveGame(socket: socketio.Socket, rooms: Rooms) {

    socket.on('leave-game', () => {
        rooms.deleteRoomByFirstPlayerId(socket.id)
        console.log(rooms)
    })

}
