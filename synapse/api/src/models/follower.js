import meNexus from "../../config/mysql.js"
import {getGlobalUsersDB} from "#src/orbitdb/globalUsers.js";

export const followUser = async (publicKey, followedPublicKey) => {
    try {
        const db = await getGlobalUsersDB();
        const [followedUser] = await db.query(doc => doc._id === followedPublicKey);
        const [followingUser] = await db.query(doc => doc._id === publicKey);
        console.log('followedUser query response: clear', followedUser);

        if (!followedUser.followers.includes(publicKey)) {
            followedUser.followers.push(publicKey);
            followingUser.following.push(followedPublicKey);
            await db.put(followedUser);
            await db.put(followingUser);
            return true;
        }
    } catch (err) {
        console.error('Error following user: ', err);
    }
}

export const unfollowUser = async (publicKey, followedPublicKey) => {
    try {
        const db = await getGlobalUsersDB();
        const [followedUser] = await db.query(doc => doc._id === followedPublicKey);
        const [followingUser] = await db.query(doc => doc._id === publicKey);

        // Safely push if not already following
        if (followedUser.followers.includes(publicKey)) {
            followedUser.followers = followedUser.followers.filter(f => f !== publicKey);
            await db.put(followedUser); // This saves the change
        }
        if (followingUser.followers.includes(followedPublicKey)) {
            followingUser.followers = followingUser.followers.filter(f => f !== followedPublicKey);
            await db.put(followingUser); // This saves the change
        }
    } catch (err) {
        console.error('Error following user: ', err);
    }
}

export const followCheck = async (publicKey, followedPublicKey) => {
    try {
        const db = await getGlobalUsersDB();
        const [followedUser] = await db.query(doc => doc._id === followedPublicKey);

        if (!followedUser || !Array.isArray(followedUser.followers)) {
            return false;
        }

        const isFollowing = followedUser.followers.includes(publicKey);
        return { isFollowing };
    } catch (err) {
        console.error('Error during followCheck: ', err);
        return { isFollowing: false };
    }
};


export const getFollowerCount = async (publicKey) => {
    try {
        const db = await getGlobalUsersDB();
        const [user] = await db.query(doc => doc._id === publicKey);
        return user.followers.length;
    } catch (err) {
        console.error('Error fetching follower count:', err);
    }
}

export const getFollowingCount = async (publicKey) => {
    try {
        const db = await getGlobalUsersDB();
        const [user] = await db.query(doc => doc._id === publicKey);
        return user.following.length;
    } catch (err) {
        console.error('Error fetching follower count:', err);
    }
}

export default {
    followUser,
    unfollowUser,
    followCheck,
    getFollowerCount,
    getFollowingCount

}