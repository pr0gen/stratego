import socketio from "socket.io";
import {Rooms} from "../../src/Rooms";


export default function game(socket: socketio.Socket, rooms: Rooms) {

    socket.on('-game', () => {

    })

}
