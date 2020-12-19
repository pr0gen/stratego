import {Rooms} from "../../src/Rooms";
import socketio from "socket.io";

export default function joinGame(socket: socketio.Socket, rooms: Rooms) {

    socket.on('join-game', (code: string) => {

        let room = rooms.getRoomByCode(code)

        if (room !== undefined) {
            room.addSecondPlayer(socket.id)
            console.log(rooms)
            socket.emit('response-join-game', {
                'valid': true
            })
            socket.to(room.firstPlayerId).emit('player-found')
        } else {
            console.log('game not found : ' + code)
        }

    })

}
