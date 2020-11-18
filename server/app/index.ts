// @ts-ignore
import generateCode from "../utils/generateCode";

import {Player} from '../src/Player'
import {PlayerState} from "../src/PlayerState";

export default function run(app, io, options): void {

// Express
    app.get('/', (req, res) => {
        res.sendFile('index.html', options)
    })

// Socket

    let players :Player[] = [];

    io.on('connection', socket => {

        // When client connect
        console.log('new player connected ' + socket.id)
        let player = new Player(socket.id, PlayerState.Waiting)
        players.push(player)

    })


}
