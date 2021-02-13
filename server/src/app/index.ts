import {PlayerState} from "../structures/PlayerState";
import {Rooms} from "../structures/Rooms";
import {Players} from "../structures/Players";
import startSockets from "../routes/socket";
import express from 'express'
import socketio from "socket.io";
const signale = require('signale');



export default async function run(app: express.Express, io: socketio.Server, options: { root: string }): Promise<void> {

// Express

    app.get('/', (req, res) => {
        res.sendFile('index.html', options)
    })

// Socket

    let players = new Players()
    let rooms = new Rooms()

    io.on('connection', socket => {
        signale.watch('new player connected ' + socket.id)
        players.addPlayer(socket.id, PlayerState.Waiting)
        startSockets(socket, players, rooms)
    })



}
