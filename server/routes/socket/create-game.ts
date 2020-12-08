import generateCode from "../../utils/generateCode";
import {Socket} from "socket.io";
import {Room} from "../../src/Room";
import {Rooms} from "../../src/Rooms";
import {RoomState} from "../../src/RoomState";

export default function createGame(socket: Socket, rooms : Rooms) {

    socket.on('create-game', () => {
        const code = generateCode()
        let room = new Room();

        room.firstPlayerId = socket.id
        room.code = code.toString()
        room.state = RoomState.waitingSecondPlayer

        rooms.addRoom(room)
        socket.emit('response-create-game', code)
    })


}
