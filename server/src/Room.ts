import {RoomState} from "./RoomState";
import {Player} from "./Player";
import {GameEngine} from "./GameEngine/GameEngine";
import {boardParser} from "../utils/boardParser";
import { PlayerState } from "./PlayerState";

export class Room {

    firstPlayer: Player
    secondPlayer: Player
    code: string
    state: RoomState
    gameEngine :GameEngine

    constructor() {
        this.firstPlayer = new Player()
        this.secondPlayer = new Player()
        this.code = ''
        this.state = RoomState.Waiting
        this.gameEngine = new GameEngine()
    }

    addSecondPlayer(secondPlayerId : string) {
        this.secondPlayer.id = secondPlayerId
        this.state = RoomState.GameStarted
    }

    createBoard() :void {
        this.gameEngine.createGame(this.firstPlayer.id, this.secondPlayer.id)
    }

    async getBoard(playerId: string) {

        const color = this.getPlayerColor(playerId)

        const board = await this.gameEngine.getBoard(color)
        return boardParser(board.cases)
    }

    async getAvailableMoves(playerId: string, x:number, y:string) {
        const color = this.getPlayerColor(playerId)
        return await this.gameEngine.getAvailableMoves(color, x, y)
    }

    getPlayerColor(playerId: string) :string {
        return this.firstPlayer.id === playerId
            ? 'Red'
            : 'Blue'
    }

    setMove(playerId: string, from_x :number, from_y: string, to_x :number, to_y: string) {
        const reponse = this.gameEngine.setMove(playerId, from_x, from_y, to_x, to_y)
    }

    async aiPlay() {
        const reponse = await this.gameEngine.aiSetMove(this.secondPlayer.aiName)
    }

    playerCanMove(playerId: string) :boolean{

        if (playerId === this.firstPlayer.id && this.secondPlayer.state == PlayerState.IsAI) {
            return true
        }

        if (playerId === this.firstPlayer.id && this.state === RoomState.FirstPlayerCanPlay) {
            return true
        }
        if (playerId === this.secondPlayer.id && this.state === RoomState.SecondPlayerCanPlay) {
            return true
        }
        return false
    }

    switchTurn() :void {
        if (this.state === RoomState.FirstPlayerCanPlay) {
            this.state = RoomState.SecondPlayerCanPlay
        }

        else if (this.state === RoomState.SecondPlayerCanPlay) {
            this.state = RoomState.FirstPlayerCanPlay
        }
    }

    getOtherPlayer(playerId:string) :Player{
        return this.firstPlayer.id === playerId
            ? this.secondPlayer
            : this.firstPlayer

    }

}
