// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import the Notification model
import Notification from '../models/notification.js'

export const createNotification = async (req, res) => {
    console.log("create notification called for publicKey: ", req.body.public_key);
    console.log("createNotification req: ", req.body);

    const { public_key, actor_public_key, resource_type, resource_id, action } = req.body;

    if(!public_key || !actor_public_key || !resource_type || !resource_id || !action) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const result = await Notification.createNotification(public_key, actor_public_key, resource_type, resource_id, action);
    return res.status(200).json(result);
}

export const getNotifications = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    console.log("/getNotifications called for publicKey: ", publicKey);

    if (!publicKey) {
        return res.status(400).json({ error: 'No authorized user' });
    }

    const results = await Notification.getNotifications(publicKey);
    return res.status(200).json(results);

}

export const setNotificationAsRead = async (req, res) => {
    const { notification_id } = req.body;

    if (!notification_id) {
        return res.status(400).json({ error: "Notification ID is required." });
    }

    const result = await Notification.setNotificationAsRead(notification_id);

    return res.status(200).json(result);
}

export default {
    createNotification,
    getNotifications,
    setNotificationAsRead,
}