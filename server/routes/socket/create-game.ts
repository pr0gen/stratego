import generateCode from "../../utils/generateCode";
import {Room} from "../../src/Room";

export default function createGame(socket, rooms) {

    socket.on('create-game', () => {
        const code = generateCode()

        let room = new Room();
        room.firstPlayerId = socket.id
        room.code = code.toString()

        console.log('create-game : ' + socket.id)

        rooms.addRoom(room)

        socket.emit('response-create-game', code)
    })


}
