// Import the Notification model
const Notification = require('../models/Notification');

exports.createNotification = async (req, res) => {
    const { user_id, actor_id, resource_type, resource_id, action } = req.body;
    console.log("createNotification req: ", req);

    if(!user_id || !actor_id || !resource_type || !resource_id || !action) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const result = await Notification.createNotification(user_id, actor_id, resource_type, resource_id, action);
    return res.status(200).json(result);
}

exports.getNotifications = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { user_id } = req.session.user;
    console.log("/getNotifications called for user_id: ", user_id);

    if (!user_id) {
        return res.status(400).json({ error: 'No authorized user' });
    }

    const results = await Notification.getNotifications(user_id);
    return res.status(200).json(results);

}

exports.setNotificationAsRead = async (req, res) => {
    const { notification_id } = req.body;

    if (!notification_id) {
        return res.status(400).json({ error: "Notification ID is required." });
    }

    const result = await Notification.setNotificationAsRead(notification_id);

    return res.status(200).json(result);
}