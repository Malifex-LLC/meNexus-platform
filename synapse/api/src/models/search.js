import meNexus from "../../config/mysql.js";
import { getUserByPublicKeyFromDB, getAllUsersFromDB } from "#src/orbitdb/globalUsers.js";


export const search = async (query, type = "mixed") => {
    const like = `%${query}%`.toLowerCase();
    const q    = query.toLowerCase();
    const results = [];

    /* ─── USERS ─────────────────────────────── */
    if (type === "users" || type === "mixed") {
        const users = await getAllUsersFromDB();

        const matched = users
            .filter(u =>
                u.handle.toLowerCase().includes(q) ||
                u.displayName.toLowerCase().includes(q)
            )
            .map(u => ({
                type: "user",
                publicKey: u.publicKey,
                handle: u.handle,
                displayName: u.displayName,
            }));

        if (type === "users") return { type: "users", results: matched };
        results.push(...matched);
    }

    /* ─── POSTS ─────────────────────────────── */
    if (type === "posts" || type === "mixed") {
        const [rows] = await meNexus
            .promise()
            .query(
                `SELECT content, post_id, public_key, created_at
         FROM   Posts
         WHERE  content LIKE ?`, [like]);

        const enriched = await Promise.all(
            rows.map(async row => {
                const author = await getUserByPublicKeyFromDB(row.public_key);
                return {
                    type: "post",
                    post_id: row.post_id,
                    publicKey: row.public_key,
                    handle: author?.handle ?? "Unknown",
                    displayName: author?.displayName ?? "Unknown",
                    content: row.content,
                    created_at: row.created_at,
                };
            })
        );

        if (type === "posts") return { type: "posts", results: enriched };
        results.push(...enriched);
    }

    return { type: "mixed", results };
};


export default {
    search
};