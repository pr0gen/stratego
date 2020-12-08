import {Rooms} from "../../src/Rooms";

export default function joinGame(socket, rooms : Rooms ) {

    socket.on('join-game', (code : string) => {

        let room = rooms.getRoomByCode(code)
        room.addSecondPlayer(socket.id)

    })

}
