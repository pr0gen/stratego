// @ts-ignore
import generateCode from "../utils/generateCode";


export default function run(app, io, options): void {

// Express
    app.get('/', (req, res) => {
        res.sendFile('index.html', options)
    })

// Socket


}
