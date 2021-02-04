import {Rooms} from "../../src/Rooms";
import socketio from "socket.io";
import {RoomState} from "../../src/RoomState";

export default function joinGame(socket: socketio.Socket, rooms: Rooms) {

    socket.on('join-game', (code: string) => {

        let room = rooms.getRoomByCode(code)
        if (room !== undefined) {

            room.createBoard()
            room.addSecondPlayer(socket.id)

            room.state = RoomState.FirstPlayerCanPlay

            socket.emit('response-join-game', {
                'valid': true
            })
            socket.to(room.firstPlayer.id).emit('player-found')

        } else {
            console.log('game not found : ' + code)
        }

    })

}
