import createGame from "./create-game";
import leaveGame from "./leave-game";
import joinGame from "./join-game";
import {Players} from "../../structures/Players";
import {Rooms} from "../../structures/Rooms";
import socketio from "socket.io";
import getAllCases from "./get-all-cases";
import getAvailableMoves from "./get-available-moves";
import movePiece from "./move-piece";
import playWithAi from "./play-with-ai";
import disconnect from "./disconnect";

export default function startSockets(socket :socketio.Socket, players: Players, rooms: Rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)
    joinGame(socket,rooms )
    getAllCases(socket, rooms)
    getAvailableMoves(socket, rooms)
    movePiece(socket, rooms)
    playWithAi(socket, rooms)
    disconnect(socket, rooms)


}
