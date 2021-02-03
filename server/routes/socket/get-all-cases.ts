import socketio from "socket.io";
import { GameEngineApi} from "../../src/GameEngine/GameEngineApi";
import {Rooms} from "../../src/Rooms";


export default function getAllCases(socket: socketio.Socket, rooms: Rooms) {

    socket.on('get-all-cases', async () => {

        const room = rooms.getRoomByPlayerId(socket.id)
        if(room === undefined) return

        const otherPlayerId = room.firstPlayer.id === socket.id
            ? room.secondPlayer.id
            : room.firstPlayer.id

        const board = await room?.getBoard(socket.id)
        const boardOtherPlayer = await room?.getBoard(otherPlayerId)

        console.log('send cases')

        socket.emit('response-get-all-cases', board)
        socket.to(otherPlayerId).emit('response-get-all-cases', boardOtherPlayer)
    })

}
