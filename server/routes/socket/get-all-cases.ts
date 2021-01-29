import socketio from "socket.io";
import { GameEngineApi} from "../../src/GameEngine/GameEngineApi";
import {Rooms} from "../../src/Rooms";


export default function getAllCases(socket: socketio.Socket, rooms: Rooms) {

    socket.on('get-all-cases', async () => {

        const room = rooms.getRoomByPlayerId(socket.id)

        const board = await room?.getBoard(socket.id)

        console.log('send cases')
        socket.emit('response-get-all-cases', board)
    })

}
