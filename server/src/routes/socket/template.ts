import socketio from "socket.io";
import {Rooms} from "../../structures/Rooms";


export default function game(socket: socketio.Socket, rooms: Rooms) {

    socket.on('', () => {

        socket.emit('response-')
    })

}
