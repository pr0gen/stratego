"use strict";
exports.__esModule = true;
// Env
var dotenv_1 = require("dotenv");
dotenv_1.config();
// Require
var express = require('express');
var socketio = require('socket.io');
var app = express();
var server = require('http').Server(app);
var morgan = require('morgan');
var io = socketio(server, { cors: { origin: '*' } });
// Constante
var port = process.env.PORT;
var devMode = process.env.MODE == 'DEV'; //bool
io.set('origins', '*:*');
var options = {
    root: __dirname + '/views/'
};
// Middleware
if (devMode) {
    app.use(morgan('dev'));
}
// Express
app.get('/', function (req, res) {
    res.sendFile('index.html', options);
});
// Socket
var games = [];
io.on('connection', function (socket) {
    console.log('connection : ' + socket.id);
    socket.on('create-game', function () {
        var code = generateCode();
        console.log('create-game ' + code);
        games[socket.id] = code;
        socket.emit('response-create-game', code);
    });
    socket.on('join-game', function (code) {
        //check if code exist
        var found = false;
        var socketOtherPlayer = null;
        for (var socketId in games) {
            if (games[socketId] == code) {
                found = true;
                socketOtherPlayer = socketId;
            }
        }
        console.log(found);
        console.log(socketOtherPlayer);
        if (found) {
            socket.emit('response-join-game', 'start game');
            socket.to(socketOtherPlayer).emit('player-found');
        }
        else {
            socket.emit('response-join-game', 'code pas bon');
        }
    });
    socket.on('disconnect', function () {
        console.log('disconnect : ' + socket.id);
    });
});
// Function
function generateCode() {
    return Math.round(Math.random() * 10000);
}
// Start listening
server.listen(port, function () {
    console.log("Server start => http://localhost:" + port);
});
