import socketio from "socket.io";
import { GameEngineApi} from "../../src/GameEngine/GameEngineApi";

export default function getAllCases( socket :socketio.Socket) {

    socket.on('get-all-cases', () => {
        const cases = GameEngineApi.initGame()
        console.log('send cases')
        socket.emit('response-get-all-cases', cases)
    })

}
