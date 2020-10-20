// Env
import {config as dotenv} from "dotenv"
dotenv()

// Require
const express = require('express')
const socketio = require('socket.io')
const app = express()
const server = require('http').Server(app)
const morgan = require('morgan')
const io = socketio(server)

// Constante
const port = process.env.PORT
const devMode = process.env.MODE == 'DEV' //bool

// Middleware
if (devMode) {
    app.use(morgan('dev'))
}


// Express
app.get('/', (req, res) => {
    res.send('stratego')
})

// Socket

io.on('connection', () => {

})


// Start listening
server.listen(port, () => {
    console.log(`Server start => http://localhost:${port}`)
})



