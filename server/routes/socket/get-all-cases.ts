import socketio from "socket.io";
import {GameEngine} from "../../src/GameEngine";

export default function getAllCases( socket :socketio.Socket) {

    socket.on('get-all-cases', () => {
        const cases = GameEngine.initGame()
        socket.emit('response-get-all-cases', cases)
    })

}
