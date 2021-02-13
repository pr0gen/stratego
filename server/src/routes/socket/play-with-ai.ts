import socketio from "socket.io";
import { PlayerState } from "../../structures/PlayerState";
import { Room } from "../../structures/Room";
import {Rooms} from "../../structures/Rooms";



export default function playWithAi(socket: socketio.Socket, rooms: Rooms) {

    socket.on('play-with-ai', (aiName) => {

        console.log("start game with : " + aiName)

        let room = new Room();
        room.firstPlayer.id = socket.id
        room.secondPlayer.state = PlayerState.IsAI
        room.secondPlayer.aiName = aiName
        room.createBoard()
        rooms.addRoom(room)

        socket.emit('response-play-with-ai',{
            aiName: aiName
        })

    })

}
