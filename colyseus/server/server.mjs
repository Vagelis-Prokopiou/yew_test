import {RelayRoom} from 'colyseus';
import {Server} from 'colyseus';

const port = parseInt(process.env.PORT, 10) || 3000;
const gameServer = new Server();

gameServer
    .define('foo', RelayRoom, {
      maxClients: 4,
      allowReconnectionTime: 120,
    }).on('create', (room) => console.log('room created:', room.roomId))
    .on('dispose', (room) => console.log('room disposed:', room.roomId))
    .on('join', (room, client) => {
      console.log('Client', client.id, 'joined room', room.roomId);
      // This works with the js client.
      setInterval(() => {
        room.broadcast('action-taken', 'an action has been taken on!' + (new Date()).toISOString());
      }, 3000);

    })
    .on('leave', (room, client) => console.log(client.id, 'left', room.roomId));

gameServer.listen(port);
console.log(`[GameServer] Listening on Port: ${port}`);