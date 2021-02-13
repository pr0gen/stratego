import socketio from "socket.io";
import {Rooms} from "../../structures/Rooms";


export default function getAvailableMoves(socket: socketio.Socket, rooms: Rooms) {

    socket.on('get-available-moves', async (position) => {

        console.log("========== get available moves ================")

        const room = rooms.getRoomByPlayerId(socket.id)
        const moves = await room?.getAvailableMoves(socket.id, position.x, position.y_letter)
        console.log(moves)
        socket.emit('response-get-available-moves', moves)
        console.log("====================================")

    })

}
