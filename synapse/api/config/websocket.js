// Import WebSocket library
const ws = require('ws')

// Store connected clients
const clients = new Map();

const createWebSocketServer = (server) => {
    const wss = new ws.Server({ noServer: true });

    // TODO Not sure if WebSocket needs CORS
    // Configure WebSocket Server for CORS
    wss.on('headers', (headers, req) => {
        headers.push('Access-Control-Allow-Origin: http://localhost:5173');
        headers.push('Access-Control-Allow-Credentials: true');
    });

    wss.on('connection', (ws, request) => {
        const urlParams = new URLSearchParams(request.url.split('?')[1]);
        const user_id = urlParams.get('user_id'); // Extract user_id from the query string
        ws.isAlive = true;

        if (!user_id) {
            console.error("WebSocket connection attempted without user_id.");
            ws.close();
            return;
        }

        console.log(`WebSocket connection established for user_id: ${user_id}`);
        clients.set(user_id, ws);

        ws.on('pong', () => {
            ws.isAlive = true; // Reset isAlive when a pong is received
        });

        ws.on('message', (message) => {
            console.log(`Received message from user ${user_id}: ${message}`);
        });

        ws.on('close', () => {
            console.log(`WebSocket connection closed for user_id: ${user_id}`);
            clients.delete(user_id);
        });
    });

    server.on('upgrade', (req, socket, head) => {
        const url = new URL(req.url, `http://${req.headers.host}`);
        const user_id = url.searchParams.get('user_id'); // Extract user_id from the query params

        if (!user_id) {
            console.error("No user_id provided in WebSocket connection");
            socket.write('HTTP/1.1 400 Bad Request\r\n\r\n');
            socket.destroy();
            return;
        }

        wss.handleUpgrade(req, socket, head, (ws) => {
            ws.user_id = user_id; // Attach user_id to the WebSocket instance
            wss.emit('connection', ws, req);
        });
    });

// Periodically ping clients to check activity
    setInterval(() => {
        wss.clients.forEach((ws) => {
            if (!ws.isAlive) {
                console.log(`Terminating inactive connection for user: ${ws.user_id}`);
                return ws.terminate();
            }

            ws.isAlive = false; // Reset and send ping
            ws.ping();
        });
    }, 10000); // Run every 10 seconds

    return wss;
};

module.exports = {
    createWebSocketServer,
    clients
}