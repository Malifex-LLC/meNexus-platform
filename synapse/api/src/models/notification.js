const meNexus = require("../../config/mysql.js");
const { clients } = require('../../config/websocket');

// Helper to send notifications via WebSocket
const sendNotification = (userId, notification) => {
    console.log(`sendNotification called with userId: ${userId} (type: ${typeof userId})`);
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

    console.log(`Sending notification to user_id: ${userId}`, notification);
    client.send(JSON.stringify(notification));
};

exports.createNotification = (user_id, actor_id, resource_type, resource_id, action) => {
    return new Promise((resolve, reject) => {
        // Fetch actor's handle from Users table
        const actorQuery = `
            SELECT handle 
            FROM Users 
            WHERE user_id = ?
        `;

        meNexus.query(actorQuery, [actor_id], (actorErr, actorResult) => {
            if (actorErr) {
                console.error("Error fetching actor handle:", actorErr);
                return reject(actorErr);
            }

            const actorHandle = actorResult[0].handle;
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
                INSERT INTO Notifications (user_id, actor_id, resource_type, resource_id, action, summary)
                VALUES (?, ?, ?, ?, ?, ?)
            `;

            meNexus.query(sql, [user_id, actor_id, resource_type, resource_id, action, summary], (err, result) => {
                if (err) {
                    console.error("Error creating notification:", err);
                    return reject(err);
                }

                // Broadcast via WebSocket
                console.log("Preparing to call sendNotification")
                console.log("Current WebSocket clients:", Array.from(clients.keys()));
                sendNotification(user_id, { summary, is_read: 0, created_at: new Date() });

                resolve(result);
            });
        });
    });
}

exports.getNotifications = (user_id) => {
    return new Promise((resolve, reject) => {
        let sql = `
            SELECT * 
            FROM Notifications 
            WHERE user_id = ? 
            AND is_read = false;
        `;

        meNexus.query(sql, [user_id], (err, results) => {
            if (err) {
                console.error("Error fetching notifications for user_id: ", user_id);
                return reject(err);
            } else {
                resolve({ notifications: results });
            }
        })

    })
}

exports.setNotificationAsRead = (notification_id) => {
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