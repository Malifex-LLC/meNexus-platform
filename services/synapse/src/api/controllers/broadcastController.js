// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { clients } from '../config/websocket.js';

export const broadcastActivity = (activity) => {
    console.log('broadcastActivity current connected clients:', clients.size);
    for (const [id, client] of clients.entries()) {
        console.log(`Client ${id} state:`, client.readyState);
        if (client.readyState === WebSocket.OPEN) {
            console.log("Broadcasting newActivity");
            client.send(JSON.stringify({
                type: 'newActivity',
                activity: activity,
            }));
        }
    }
};

export default {
    broadcastActivity,
}