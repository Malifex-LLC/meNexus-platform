// Import the Follower model
const Follower = require('../models/follower')

exports.followUser = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the follower's user ID from the session
    const { followed_id } = req.body;
    console.log("handle Follow user: ", followed_id, " for user: ", user_id);

    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const result = await Follower.followUser(user_id, followed_id);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'User not found'});
    }

    return res.status(200).json({result});
}

exports.unfollowUser = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the follower's user ID from the session
    const { followed_id } = req.body;
    console.log("handle unfollow user: ", followed_id, " for user: ", user_id);

    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const result = await Follower.unfollowUser(user_id, followed_id);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'User not found'});
    }

    return res.status(200).json({result});
}

exports.followCheck = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the current user's ID
    const { followed_id } = req.query; // ID of the user being checked
    console.log("server handling followCheck for followed_id: ", followed_id, "for user_id: ", user_id);

    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.followCheck(user_id, followed_id);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'User not found'});
    }

    return res.status(200).json(result);
}

exports.getFollowerCount = async (req, res) => {
    const { user_id } = req.query;

    if (!user_id) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.getFollowerCount(user_id);

    return res.status(200).json({result});
}

exports.getFollowingCount = async (req, res) => {
    const { user_id } = req.query;

    if (!user_id) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.getFollowingCount(user_id);

    return res.status(200).json({result});
}