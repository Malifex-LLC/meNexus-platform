// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Chat from '../models/chat.js'
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";
import path from 'path';
import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');
import { WebSocketServer } from "ws";
// Store connected clients
export const clients = new Map();

export const createWebSocketServer = (server) => {
    const wss = new WebSocketServer({ noServer: true });

    // TODO Not sure if WebSocket needs CORS
    // Configure WebSocket Server for CORS
    wss.on('headers', (headers, req) => {
        headers.push('Access-Control-Allow-Origin: http://localhost:5173');
        headers.push('Access-Control-Allow-Credentials: true');
    });

    wss.on('connection', (ws, request) => {
        const publicKey = ws.publicKey
        ws.isAlive = true;

        if (!publicKey) {
            console.error("WebSocket connection attempted without publicKey.");
            ws.close();
            return;
        }

        //console.log(`WebSocket connection established for user_id: ${publicKey}`);
        clients.set(publicKey, ws);

        ws.on('pong', () => {
            ws.isAlive = true; // Reset isAlive when a pong is received
        });

        ws.on('message', async (message) => {
            try {
                const parsed = JSON.parse(message);

                if (parsed.type === 'chatMessage') {
                    const { activeChannel, publicKey, content } = parsed;
                    console.log(`Received WebSocket message from user ${publicKey} from channel ${activeChannel}:`, content);

                    // Store in DB
                    const chat = await Chat.createChatMessage(publicKey, activeChannel, content);

                    // Create enriched message (e.g. displayName, handle)
                    const user = await getUserByPublicKeyFromDB(publicKey);
                    const config = await loadConfig(CONFIG_FILE);

                    const enriched = {
                        chat_id: chat.chatId,
                        public_key: publicKey,
                        displayName: user?.displayName || 'Unknown',
                        handle: user?.handle || 'Unknown',
                        synapsePublicKey: config.identity.publicKey,
                        synapseUrl: config.identity.synapseUrl,
                        content,
                        created_at: new Date().toISOString(),
                        activeChannel: activeChannel,
                    };

                    // Broadcast to all clients in the same activeChannel
                    console.log('Current connected clients:', clients.size);
                    for (const [id, client] of clients.entries()) {
                        console.log(`Client ${id} state:`, client.readyState);
                        if (client.readyState === 1) {
                            console.log("Broadcasting message to channel:", activeChannel);
                            client.send(JSON.stringify({
                                type: 'newChatMessage',
                                message: enriched,
                                activeChannel,
                            }));
                        }
                    }
                }

            } catch (err) {
                console.error("Error processing WebSocket message:", err);
            }
        });

        ws.on('close', () => {
            console.log(`WebSocket connection closed for publicKey: ${publicKey}`);
            clients.delete(publicKey);
        });
    });

    server.on('upgrade', (req, socket, head) => {
        const url = new URL(req.url, `http://${req.headers.host}`);
        const publicKey = url.searchParams.get('publicKey');

        if (!publicKey) {
            console.error("No publicKey provided in WebSocket connection");
            socket.write('HTTP/1.1 400 Bad Request\r\n\r\n');
            socket.destroy();
            return;
        }

        wss.handleUpgrade(req, socket, head, (ws) => {
            // Attach directly so it's available immediately
            ws.publicKey = publicKey;
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

export default {
    createWebSocketServer,
    setInterval
}