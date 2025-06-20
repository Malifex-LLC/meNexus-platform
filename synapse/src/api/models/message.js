import meNexus from "../config/mysql.js";
import { clients } from '../config/websocket.js';

// Helper to send messages via WebSocket
const sendMessage = (userId, message) => {
    console.log(`sendMessage called with userId: ${userId} (type: ${typeof userId})`);
    console.log("Current WebSocket clients:", Array.from(clients.keys()));

    const client = clients.get(String(userId)); // Ensure type consistency
    if (!client) {
        console.log(`No WebSocket client found for user_id: ${userId}`);
        return;
    }

    if (client.readyState !== WebSocket.OPEN) {
        console.log(`WebSocket client for user_id: ${userId} is not open. Current state: ${client.readyState}`);
        return;
    }

    console.log(`Sending message to user_id: ${userId}`, message);
    client.send(JSON.stringify(message));
};

export const getMessages = (conversation_id) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT * 
            FROM Messages 
            WHERE conversation_id = ?;
        `;

        meNexus.query(sql, [conversation_id], (err, results) => {
            if (err) {
                console.error('Error fetching messages', err);
                return reject(err);
            }

            resolve(results);
        })

    })
}

export const createMessage = (publicKey, conversation_id, message, participant_id, content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO Messages (conversation_id, sender_public_key, receiver_public_key, content, created_at)
            VALUES (?, ?, ?, ?, NOW())
        `;
        meNexus.query(sql, [conversation_id, publicKey, participant_id, content], (err, results) => {
            if (err) {
                console.error('Error creating message', err);
                return reject(err);
            }
            console.log('/createMessage results:', results);
            // Fetch the inserted message with the generated timestamp
            const fetchSql = `
                SELECT *
                FROM Messages
                WHERE message_id = ?;
             `;
            const messageId = results.insertId;
            meNexus.query(fetchSql, [messageId], (fetchErr, fetchedResults) => {
                if (fetchErr) {
                    console.error('Error fetching the created message:', fetchErr);
                    return reject(fetchErr);
                }
                const fullMessage = fetchedResults[0];
                console.log('Fetched full message:', fullMessage);

                // Send the full message via WebSocket
                sendMessage(participant_id, fullMessage);

                // Respond to the sender
                resolve(fullMessage);
            });
        });
    })
}

export const setMessageAsRead = (conversation_id) => {
    return new Promise((resolve, reject) => {
        const sql = `
            UPDATE Messages 
            SET is_read = TRUE 
            WHERE conversation_id = ?;
        `;

        meNexus.query(sql, [conversation_id], (err, results) => {
            if (err) {
                console.error('Error updating messages as read', err);
                return reject(err);
            }

            console.log('Messages marked as read for conversation_id:', conversation_id);
            resolve({ message: 'Messages marked as read', affectedRows: results.affectedRows })
        });
    })
}

export default {
    getMessages,
    createMessage,
    setMessageAsRead
}