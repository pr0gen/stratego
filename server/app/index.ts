import {PlayerState} from "../src/PlayerState";
import {Rooms} from "../src/Rooms";
import {Players} from "../src/Players";
import startSockets from "../routes/socket";


export default function run(app, io, options): void {

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
