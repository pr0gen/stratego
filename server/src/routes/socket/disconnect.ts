import socketio from "socket.io";
import {Rooms} from "../../structures/Rooms";
const log = require('signale');


export default function disconnect(socket: socketio.Socket, rooms: Rooms) {

    socket.on('disconnect', () => {
        log.info('deconnection player : ' + socket.id)
    })

}
