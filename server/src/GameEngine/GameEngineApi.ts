import axios from "axios";

export class GameEngineApi {

    private static _urlBase: string = 'http://127.0.0.1:8000'

    static set urlBase(value: string) {
        this._urlBase = value;
    }

    static async helloWorld() {
        const {data: response} = await axios.get(this._urlBase + '/')
        return response
    }

    static async getGame(uuid: string, color: string) {
        const {data: response} = await axios.get(`http://127.0.0.1:8000/game/${uuid}/${color}`)
        return response
    }

    static async createGame(player_id_1: string, player_id_2: string, type: string) {

        const board = [
            []
        ]

        const {data: response} = await axios.post('http://127.0.0.1:8000/create-game',
            {
                player_id_1,
                player_id_2,
                type,
                board
            }
        )
        return response.uuid
    }

    static async getAvailableMove(uuid:string, player_color:string){
        const {data: response} = await axios.get(`http://127.0.0.1:8000/moves/${player_color}/${uuid}`)
        return response
    }

    static async setMove(uuid:string, player_id:string, coordinate_from : (string|number)[], coordinate_to : (string|number)[] ){
        const {data: response} = await axios.post(`http://127.0.0.1:8000/moves`, {
            uuid,
            player_id,
            coordinate_from,
            coordinate_to
        })
        return response
    }


    static initGame() {
        return [
            'B', 'B', 'B', 'B', 'B', 'Maj', 'Maj', 'Maj', 'G', 'G', 'G', 'G',
            'Capt', 'Capt', 'Capt', 'Capt', 'Ser', 'Ser', 'Ser', 'L', 'L', 'L', 'L'
        ]
    }


}
