import createGame from "./create-game";
import leaveGame from "./leave-game";
import joinGame from "./join-game";
import {Players} from "../../src/Players";
import {Rooms} from "../../src/Rooms";
import socketio from "socket.io";
import getAllCases from "./get-all-cases";

export default function startSockets(socket :socketio.Socket, players: Players, rooms: Rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)
    joinGame(socket, rooms)
    getAllCases(socket)

}
