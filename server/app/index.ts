import {PlayerState} from "../src/PlayerState";
import {Rooms} from "../src/Rooms";
import {Players} from "../src/Players";
import startSockets from "../routes/socket";
import express from 'express'
import socketio from "socket.io";


export default async function run(app: express.Express, io: socketio.Server, options: { root: string }): Promise<void> {


// Express

    app.get('/', (req, res) => {
        res.sendFile('index.html', options)
    })

// Socket

    let players = new Players()
    let rooms = new Rooms()

    io.on('connection', socket => {

        console.log('new player connected ' + socket.id)
        players.addPlayer(socket.id, PlayerState.Waiting)
        startSockets(socket, players, rooms)

    })

}
