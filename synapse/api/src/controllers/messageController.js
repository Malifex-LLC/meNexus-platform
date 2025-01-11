// Import Message model
const Message = require("../models/message");

exports.getMessages = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the current user's ID
    const { conversation_id } = req.query;

    if(!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const results = await Message.getMessages(conversation_id);

    return res.status(200).json(results);
}

exports.createMessage = async (req, res) => {
    console.log('/createMessage called');
    console.log('/createMessage req.body:', req.body);
    console.log('/createMessage req.query:', req.query);

    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the current user's ID
    const { conversation_id } = req.body;
    const { message } = req.body;
    const participant_id = message.participant_id;
    const content = message.content;

    if(!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    if (!conversation_id || !content) {
        return res.status(400).json({ error: 'Missing conversation_id or content' });
    }

    const result = await Message.createMessage(user_id, conversation_id, message, participant_id, content);
    return res.status(200).json(result);
}

exports.setMessagesAsRead = async (req, res) => {
    console.log('/setMessagesAsRead called for: ', req.body);
    const { conversation_id } = req.body;

    const results = await Message.setMessageAsRead(conversation_id);
    return res.status(200).json(results);
}