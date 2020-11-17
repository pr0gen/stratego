// @ts-ignore
import generateCode from "../utils/generateCode";

import { joinGame } from '../routes/socket'

export default function run(app, io, options): void {

// Express
    app.get('/', (req, res) => {
        res.sendFile('index.html', options)
    })

// Socket

    let games: any[];
    games = [];

    io.on('connection', (socket) => {
        console.log('connection : ' + socket.id)

        socket.on('create-game', () => {
            const code = generateCode()
            console.log('create-game ' + code)
            games[socket.id] = code
            socket.emit('response-create-game', code)
        })

        socket.on('join-game', (code) => {

            //check if code exist
            let found = false
            let socketOtherPlayer = null

            for (let socketId in games) {
                if (games[socketId] == code) {
                    found = true
                    socketOtherPlayer = socketId
                }
            }
            console.log(found)
            console.log(socketOtherPlayer)
            if (found) {
                socket.emit('response-join-game', 'start game')
                socket.to(socketOtherPlayer).emit('player-found')
            } else {
                socket.emit('response-join-game', 'code pas bon')
            }

        })


        socket.on('disconnect', () => {
            console.log('disconnect : ' + socket.id)
        })


    })


}
