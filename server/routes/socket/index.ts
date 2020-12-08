import createGame from "./create-game";
import leaveGame from "./leave-game";
import joinGame from "./join-game";

export default function startSockets(socket, players, rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)
    joinGame(socket, rooms)

}
