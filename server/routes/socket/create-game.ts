import generateCode from "../../utils/generateCode";
import {Socket} from "socket.io";
import {Room} from "../../src/Room";
import {Rooms} from "../../src/Rooms";

export default function createGame(socket: Socket, rooms : Rooms) {

    socket.on('create-game', () => {
        const code = generateCode()

        let room = new Room();
        room.firstPlayerId = socket.id
        room.code = code.toString()

        console.log('create-game : ' + socket.id)

        rooms.addRoom(room)

        socket.emit('response-create-game', code)

        console.log(rooms)
    })


}
