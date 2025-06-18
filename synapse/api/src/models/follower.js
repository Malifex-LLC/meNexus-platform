import meNexus from "../../config/mysql.js"

// Logic to follow a user
export const followUser = (user_id, followed_id) => {
    return new Promise((resolve, reject) => {
        const sql = 'INSERT INTO Followers (follower_id, followed_id) VALUES (?, ?)';

        meNexus.query(sql, [user_id, followed_id], (err, result) => {
            if (err) {
                console.error('Error adding follow:', err.message);
                return reject(err);
            }

            resolve(result)
        });
    })
}

// Logic to unfollow a user
export const unfollowUser = (user_id, followed_id) => {
    return new Promise((resolve, reject) => {
        const sql = 'DELETE FROM Followers WHERE follower_id = ? AND followed_id = ?';
        meNexus.query(sql, [user_id, followed_id], (err, result) => {
            if (err) {
                console.error('Error removing follow:', err.message);
                return reject(err);
            }

            resolve(result)
        });
    })
}

// Logic to check if a user is following another user
export const followCheck = (publicKey, followedPublicKey) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT * 
            FROM Followers 
            WHERE follower_public_key = ? AND followed_public_key = ?
         `;

        meNexus.query(sql, [publicKey, followedPublicKey], (err, results) => {
            if (err) {
                console.error('Error checking follow status:', err);
                return reject(err);
            }

            const isFollowing = results.length > 0; // If a record exists, the user is following
            console.log("isFollowing: ", isFollowing);
            resolve({isFollowing});
        });
    });
}

export default {
    followUser,
    unfollowUser,
    followCheck,
}