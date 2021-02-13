import socketio from "socket.io";
import { GameEngineApi} from "../../structures/GameEngine/GameEngineApi";
import { PlayerState } from "../../structures/PlayerState";
import {Rooms} from "../../structures/Rooms";


export default function getAllCases(socket: socketio.Socket, rooms: Rooms) {

    socket.on('get-all-cases', async () => {

        const room = rooms.getRoomByPlayerId(socket.id)
        if(room === undefined) return

        console.log('send cases')

        const board = await room?.getBoard(socket.id)
        socket.emit('response-get-all-cases', board, room.playerCanMove(socket.id))

        const otherPlayer = room.getOtherPlayer(socket.id)
        if(otherPlayer.state === PlayerState.IsAI) return

        const boardOtherPlayer = await room?.getBoard(otherPlayer.id)
        socket.to(otherPlayer.id).emit('response-get-all-cases', boardOtherPlayer,room.playerCanMove(otherPlayer.id))
    })

}
