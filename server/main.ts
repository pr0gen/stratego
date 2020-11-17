// Env
import {config as dotenv} from 'dotenv'
import run from './app'

dotenv()

// Require
const express = require('express')
const socketio = require('socket.io')
const app = express()
const server = require('http').Server(app)
const morgan = require('morgan')
const io = socketio(server, { cors: { origin: '*' } })


// Constante
const port = process.env.PORT
const devMode = process.env.NODE_ENV !== 'production' //bool
io.set('origins', '*:*');

const options = {
    root: __dirname + '/views/'
}

// Middleware
if (devMode) {
    app.use(morgan('dev'))
}

run(app,io,options)

// Function



// Start listening
server.listen(port, () => {
    console.log(`Server start => http://localhost:${port}`)
})
