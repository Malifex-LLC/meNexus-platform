import meNexus from "../../config/mysql.js"

// Logic to follow a user
export const followUser = (publicKey, followed_id) => {
    return new Promise((resolve, reject) => {
        const sql = 'INSERT INTO Followers (follower_public_key, followed_public_key) VALUES (?, ?)';

        meNexus.query(sql, [publicKey, followed_id], (err, result) => {
            if (err) {
                console.error('Error adding follow:', err.message);
                return reject(err);
            }

            resolve(result)
        });
    })
}

// Logic to unfollow a user
export const unfollowUser = (publicKey, followed_id) => {
    return new Promise((resolve, reject) => {
        const sql = 'DELETE FROM Followers WHERE follower_public_key = ? AND followed_public_key = ?';
        meNexus.query(sql, [publicKey, followed_id], (err, result) => {
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

export const getFollowerCount = (publicKey) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT COUNT(*) AS follower_count 
            FROM Followers 
            WHERE followed_public_key = ?
        `;

        meNexus.query(sql, [publicKey], (err, result) => {
            if (err) {
                console.error('Error fetching follower count:', err);
                return reject(err);
            }

            resolve(result[0])
        })
    })
}

export const getFollowingCount = (publicKey) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT COUNT(*) AS following_count 
            FROM Followers 
            WHERE follower_public_key = ?
        `;

        meNexus.query(sql, [publicKey], (err, result) => {
            if (err) {
                console.error('Error fetching following count:', err);
                return reject(err);
            }

            resolve(result[0])
        })
    })
}

export default {
    followUser,
    unfollowUser,
    followCheck,
    getFollowerCount,
    getFollowingCount

}