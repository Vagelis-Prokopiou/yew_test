import * as Colyseus from "colyseus.js";

let client = new Colyseus.Client("ws://localhost:3000");

client.joinOrCreate("foo").then(room => {
    console.log(room.sessionId, "joined", room.name);
}).catch(e => {
    console.log("JOIN ERROR", e);
});


/*const relay = await client.joinOrCreate("your_relayed_room", {
  name: "This is my name!"
});*/