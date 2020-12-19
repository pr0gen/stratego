import createGame from "./create-game";
import leaveGame from "./leave-game";
import joinGame from "./join-game";
import {Players} from "../../src/Players";
import {Rooms} from "../../src/Rooms";
import socketio from "socket.io";

export default function startSockets(socket :socketio.Socket, players: Players, rooms: Rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)
    joinGame(socket, rooms)

}
