import * as Colyseus from "colyseus.js";

let client = new Colyseus.Client("ws://localhost:3000");

client
    .joinOrCreate("foo")
    .then(room => {
        console.log(room.sessionId, "joined", room.name);
        room.onMessage("*", (message_type, data) => {
            console.log('message_type', message_type);
            console.log('data', data);
        });
    })
    .catch(e => {
        console.log("JOIN ERROR", e);
    });