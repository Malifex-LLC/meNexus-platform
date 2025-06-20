// Import the Comment model
import Comment from '../models/comment.js';

export const createComment = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    const { publicKey } = req.session.user
    const { resource_type, resource_id, content } = req.body;

    const result = await Comment.createComment(publicKey, resource_type, resource_id, content);

    if (result.affectedRows === 0) {
        return res.status(500).json({error: 'Failed to create a comment'});
    }

    return res.status(200).json(result);
}

export const updateComment = async (req, res) => {
    const comment_id = req.params.comment_id;
    const updatedContent = req.body.content;

    const result = await Comment.updateComment(comment_id, updatedContent);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'Comment not found'});
    }

    return res.status(200).json(result);
}

export const deleteComment = async (req, res) => {
    const comment_id = req.params.comment_id;

    const result = await Comment.deleteComment(comment_id);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'Comment not found'});
    }

    return res.status(200).json(result);
}

export const getComments = async (req, res) => {
    const {resource_type, resource_id } = req.query;

    if (!resource_id || resource_id.trim() === "") {
        return res.status(400).json({ error: "Invalid getComments query." });
    }

    const results = await Comment.getComments(resource_type, resource_id);
    return res.status(200).json(results);
}

export default {
    createComment,
    updateComment,
    deleteComment,
    getComments
}