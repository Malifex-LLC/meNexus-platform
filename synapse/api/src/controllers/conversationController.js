// Import Conversation model
const Conversation = require('../models/conversation');

exports.getConversations = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { user_id } = req.session.user; // Get the current user's ID
    console.log('/getConversations called for user_id: ', user_id);

    if(!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const results = await Conversation.getConversations(user_id);
    res.status(200).json(results);
}

exports.createConversation = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { user_id } = req.session.user;
    console.log("/createConversation called for participants");

    const result = await Conversation.createConversation(user_id);

    if(result.affectedRows > 0) {
        res.status(500).json({error: "Failed to create conversation"});
    }

    return res.status(200).json(result);
}

exports.updateConversationParticipants = async (req, res) => {
    const newParticipantsHandle = req.body.participants;
    const conversation_id = req.body.conversation_id;
    console.log("/updateConversationParticipants called for participant: ", newParticipantsHandle, ' and conversation_id: ', conversation_id);

    try {
        const result = await Conversation.updateConversationParticipants(conversation_id, newParticipantsHandle);

        return res.status(200).json(result);
    } catch (error) {
        console.error("Error updating conversation participants", error);
        return res.status(400).json({ error: error.message });
    }
}