import socketio from "socket.io";
import { PlayerState } from "../../src/PlayerState";
import { Room } from "../../src/Room";
import {Rooms} from "../../src/Rooms";



export default function playWithAi(socket: socketio.Socket, rooms: Rooms) {

    socket.on('play-with-ai', (aiName) => {

        console.log(aiName)

        let room = new Room();
        room.firstPlayer.id = socket.id
        room.secondPlayer.state = PlayerState.IsAI
        room.createBoard()

        socket.emit('response-play-with-ai',{
            aiName: aiName
        })

    })

}
