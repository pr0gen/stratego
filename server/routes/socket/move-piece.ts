import socketio from "socket.io";
import {PlayerState} from "../../src/PlayerState";
import {Rooms} from "../../src/Rooms";


export default function movePiece(socket: socketio.Socket, rooms: Rooms) {

    socket.on('move-piece', async (from, to) => {

        console.log("========== Move piece ================")
        const room = rooms.getRoomByPlayerId(socket.id)
        if (room === undefined) return

        const otherPlayer = room.getOtherPlayer(socket.id)
        const moves = await room?.setMove(socket.id, from.x, from.y_letter, to.x, to.y_letter)

        console.log(from)
        console.log(to)

        if (otherPlayer.state == PlayerState.IsAI) {
            const moves = await room?.aiPlay()
        } else {
            room?.switchTurn()
        }

        console.log("====================================")
        socket.emit('response-move-piece', moves)
    })

}
