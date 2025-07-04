// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import the Follower model
import Follower from '../models/follower.js'

export const followUser = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { publicKey } = req.session.user;
    const { followedPublicKey } = req.body;
    console.log("handle Follow user: ", followedPublicKey, " for user: ", publicKey);

    if (!publicKey || !followedPublicKey) {
        return res.status(400).json({ error: 'Invalid request data' });
    }
    const result = await Follower.followUser(publicKey, followedPublicKey);
    if (result === false) {
        return res.status(404).json({ error: 'Failed to follow user: ', followedPublicKey });
    }
    return res.status(200).json({result});
}

export const unfollowUser = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { publicKey } = req.session.user; // Get the follower's user ID from the session
    const { followedPublicKey } = req.body;
    console.log("handle unfollow user: ", followedPublicKey, " for user: ", publicKey);

    if (!publicKey || !followedPublicKey) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const result = await Follower.unfollowUser(publicKey, followedPublicKey);

    return res.status(200).json({result});
}

export const followCheck = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { publicKey } = req.session.user; // Get the current user's ID
    const { followedPublicKey } = req.query; // ID of the user being checked
    console.log("api handling followCheck for followedPublicKey: ", followedPublicKey, "for publicKey: ", publicKey);

    if (!publicKey || !followedPublicKey) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.followCheck(publicKey, followedPublicKey);

    return res.status(200).json(result);
}

export const getFollowerCount = async (req, res) => {
    const { publicKey } = req.query;

    if (!publicKey) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.getFollowerCount(publicKey);
    console.log('getFollowerCount result: ', result);

    return res.status(200).json({result});
}

export const getFollowingCount = async (req, res) => {
    const { publicKey } = req.query;

    if (!publicKey) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const result = await Follower.getFollowingCount(publicKey);
    console.log('getFollowingCount result: ', result);

    return res.status(200).json({result});
}

export default {
    followUser,
    unfollowUser,
    followCheck,
    getFollowerCount,
    getFollowingCount
}