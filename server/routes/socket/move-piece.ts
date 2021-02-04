import socketio from "socket.io";
import {Rooms} from "../../src/Rooms";


export default function movePiece(socket: socketio.Socket, rooms: Rooms) {

    socket.on('move-piece', async (from, to) => {

        console.log("========== Move piece ================")
        const room = rooms.getRoomByPlayerId(socket.id)
        room?.switchTurn()

        console.log(from)
        console.log(to)


        const moves = await room?.setMove(socket.id, from.x, from.y_letter, to.x, to.y_letter )
        console.log("====================================")
        socket.emit('response-move-piece', moves)
    })

}
