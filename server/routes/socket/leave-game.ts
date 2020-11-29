
export default function leaveGame(socket, rooms) {

    socket.on('leave-game', () => {
        rooms.deleteRoomByFirstPlayerId(socket.id)
    })

}
