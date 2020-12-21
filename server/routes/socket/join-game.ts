import {Rooms} from "../../src/Rooms";

export default function joinGame(socket, rooms : Rooms ) {

    socket.on('join-game', (code : string) => {

        let room = rooms.getRoomByCode(code)

        if (room !== null) {
            room.addSecondPlayer(socket.id)
            console.log(rooms)
            socket.emit('response-join-game', {
                'valid' : true
            })
            socket.to(room.firstPlayerId).emit('player-found')
        }

        else {
            console.log('game not found : ' + code)
        }

    })

}
