import generateCode from "../../utils/generateCode";
import socketio, {Socket} from "socket.io";
import {Room} from "../../structures/Room";
import {Rooms} from "../../structures/Rooms";
import {RoomState} from "../../structures/RoomState";
import {Log} from "../../utils/Log";


export default function createGame( socket :socketio.Socket, rooms: Rooms) {

    const log = Log.getInstance()

    socket.on('create-game', () => {
        const code = generateCode()
        log.socket("create games : " + code)
        let room = new Room();
        room.firstPlayer.id = socket.id
        room.code = code.toString()
        room.state = RoomState.WaitingSecondPlayer

        rooms.addRoom(room)
        socket.emit('response-create-game', code)
        // console.log(rooms)
    })


}
