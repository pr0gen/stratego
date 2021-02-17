import { config as dotenv } from 'dotenv'
import { env } from './utils/template/env'

dotenv()

// Require
import express from 'express'
import socketio from 'socket.io'
const app = express()
import { Server } from 'http'
import morgan from 'morgan'
import run from "./app";
import {Log} from "./utils/Log";
const signale = require('signale');
const boxen = require('boxen');
Log.init()



const server = new Server(app)

// @ts-ignore
const io = socketio(server, { cors: { origin: '*' } })

// Constante
const port = process.env.PORT
const devMode = process.env.NODE_ENV !== 'production' //bool
//io.set('origins', '*:*');

const options = {
    root: __dirname + '/views/'
}

// Middleware
if (devMode) {
    app.use(morgan('dev'))
}

run(app,io,options)

// Start listening
server.listen(port, () => {
    console.log(boxen(`Stratego Server 2.0`, {padding: 1}));
    signale.success(`server listen => http://localhost:${port}`)
})
