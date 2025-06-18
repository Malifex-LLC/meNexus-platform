import meNexus from "../../config/mysql.js";
import  { clients } from '../../config/websocket.js';
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

// Helper to send notifications via WebSocket
const sendNotification = (publicKey, notification) => {
    console.log(`sendNotification called with userId: ${publicKey} (type: ${typeof publicKey})`);
    console.log("Current WebSocket clients:", Array.from(clients.keys()));

    const client = clients.get(String(publicKey)); // Ensure type consistency
    if (!client) {
        console.log(`No WebSocket client found for user_id: ${publicKey}`);
        return;
    }

    if (client.readyState !== WebSocket.OPEN) {
        console.log(`WebSocket client for user_id: ${publicKey} is not open. Current state: ${client.readyState}`);
        return;
    }

    console.log(`Sending notification to user_id: ${publicKey}`, notification);
    client.send(JSON.stringify(notification));
};

export const createNotification = async (public_key, actor_public_key, resource_type, resource_id, action) => {
    // Fetch actor's handle from globalUsers
    const actor = await getUserByPublicKeyFromDB(actor_public_key);
    return new Promise((resolve, reject) => {
        console.log('createNotification for actor: ', actor);
        const actorHandle = actor.handle;
        // Generate notification summary based on resource_type
        let summary = "";
        switch (resource_type) {
            case "POST":
                if (action === "COMMENT") {
                    summary = `${actorHandle} commented on your post!`;
                } else if (action === "LIKE") {
                    summary = `${actorHandle} liked your post!`;
                }
                break;
            case "COMMENT":
                if (action === "LIKE") {
                    summary = `${actorHandle} liked your comment!`;
                }
                break;
            case "FOLLOW":
                summary = `${actorHandle} followed you!`;
                break;
            default:
                return reject({error: "Unsupported resource_type or action"});
        }

        const sql = `
            INSERT INTO Notifications (public_key, actor_public_key, resource_type, resource_id, action, summary)
            VALUES (?, ?, ?, ?, ?, ?)
        `;
        meNexus.query(sql, [public_key, actor_public_key, resource_type, resource_id, action, summary], (err, result) => {
            if (err) {
                console.error("Error creating notification:", err);
                return reject(err);
            }
            // Broadcast via WebSocket
            console.log("Preparing to call sendNotification")
            console.log("Current WebSocket clients:", Array.from(clients.keys()));
            sendNotification(public_key, {summary, is_read: 0, created_at: new Date()});
            resolve(result);
        });
    });
}

export const getNotifications = (publicKey) => {
    return new Promise((resolve, reject) => {
        let sql = `
            SELECT * 
            FROM Notifications 
            WHERE public_key = ? 
            AND is_read = false;
        `;

        meNexus.query(sql, [publicKey], (err, results) => {
            if (err) {
                console.error("Error fetching notifications for publicKey: ", publicKey);
                return reject(err);
            } else {
                resolve({ notifications: results });
            }
        })

    })
}

export const setNotificationAsRead = (notification_id) => {
    return new Promise((resolve, reject) => {
        let sql = `
            UPDATE Notifications 
            SET is_read = 1 
            WHERE notification_id = ?
        `;

        meNexus.query(sql, [notification_id], (err, result) => {
            if (err) {
                console.error("Error updating notification:", err);
                return reject(err);
            }

            resolve(result);
        });
    })
}

export default {
    createNotification,
    getNotifications,
    setNotificationAsRead
}