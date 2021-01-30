import createGame from "./create-game";
import leaveGame from "./leave-game";
import joinGame from "./join-game";
import {Players} from "../../src/Players";
import {Rooms} from "../../src/Rooms";
import socketio from "socket.io";
import getAllCases from "./get-all-cases";
import getAvailableMoves from "./get-available-moves";
import movePiece from "./move-piece";

export default function startSockets(socket :socketio.Socket, players: Players, rooms: Rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)
    joinGame(socket,rooms )
    getAllCases(socket, rooms)
    getAvailableMoves(socket, rooms)
    movePiece(socket, rooms)

}
