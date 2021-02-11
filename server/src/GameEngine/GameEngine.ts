import {GameEngineState} from "./GameEngineState";
import {GameEngineApi} from "./GameEngineApi";

export class GameEngine {

    uuid :string = ''
    state :GameEngineState

    constructor() {
        this.state = GameEngineState.NotInitialized
    }

    async createGame(player_1: string, player_2: string) {
        try {
            this.uuid = await GameEngineApi.createGame(player_1, player_2, 'human')
            console.log(this.uuid)
        } catch (e) {
            console.log("error, creation board api")
        }
    }

    async getBoard(color:string) {
        try {
            const board =  await GameEngineApi.getGame(this.uuid, color)
            return board
        } catch (e) {
            console.log("error, get board api")
        }

    }

    async getAvailableMoves(color: string, x: number, y: string) {
        type Move = {
            from_x:number,
            from_y:string,
        }

        try {
            let moves:Move[] = await GameEngineApi.getAvailableMove(this.uuid, color)
            return moves.filter(move => move.from_x === x && move.from_y === y)
        } catch (e) {
            console.log("error, get board api")
        }

    }

    async setMove(player_id:string, from_x :number, from_y: string, to_x :number, to_y: string) {

        const reponse = await GameEngineApi.setMove(this.uuid, player_id, [from_x,from_y], [to_x, to_y])
        console.log(reponse)
    }

    async aiSetMove(aiName:string) {

        const reponse = await GameEngineApi.apiSetMove(this.uuid, 'Blue', aiName)
        console.log(reponse)
    }


}
