import createGame from "./create-game";
import leaveGame from "./leave-game";

export default function startSockets(socket, players, rooms) {

    createGame(socket, rooms)
    leaveGame(socket, rooms)

}
