const meNexus = require("../../config/mysql.js");

exports.getConversations = (user_id) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT
                c.conversation_id,
                u.user_id AS participant_id,
                u.handle AS participant_handle,
                EXISTS (
                           SELECT 1
                           FROM Messages m
                           WHERE m.conversation_id = c.conversation_id AND m.is_read = FALSE
                       ) AS has_unread_messages
            
            FROM
                Conversations c
                    JOIN
                ConversationParticipants cp ON c.conversation_id = cp.conversation_id
                    JOIN
                Users u ON cp.user_id = u.user_id
                    LEFT JOIN
                Messages m ON c.conversation_id = m.conversation_id
            WHERE
                cp.conversation_id IN (
                    SELECT conversation_id
                    FROM ConversationParticipants
                    WHERE user_id = ?
                )
              AND u.user_id != ?
            GROUP BY
                c.conversation_id, participant_id
        `;

        meNexus.query(sql, [user_id, user_id], (err, results) => {
            if (err) {
                console.error('Error fetching conversations', err);
                return reject(err);
            }

            console.log('/getConversations results:', results);
            resolve(results);
        });
    });
};

exports.createConversation = (user_id) => {
    return new Promise((resolve, reject) => {
        let newConversationId;

        const newConversationSql = `
            INSERT INTO Conversations () 
            VALUES ();
    `;

        meNexus.query(newConversationSql, (err, results) => {
            if (err) {
                console.error('Error creating conversation', err);
                return reject(err);
            }

            newConversationId = results.insertId;
            console.log('newConversationId', newConversationId);

            const addSenderParticipantSql = `
                INSERT INTO 
                    ConversationParticipants (conversation_id, user_id) 
                VALUES (?, ?);
            `;

            meNexus.query(addSenderParticipantSql, [newConversationId, user_id], (err, results) => {
                if (err) {
                    console.error('Error adding sender to  ConversationParticipants', err);
                    return reject(err);
                }

                resolve({conversation_id: newConversationId})
            });
        });
    });
}

exports.updateConversationParticipants = (conversation_id, newParticipantsHandle) => {
    return new Promise((resolve, reject) => {
        const getParticipantUserIdSql = `
            SELECT user_id 
            from Users 
            WHERE handle = ?;
        `;

        meNexus.query(getParticipantUserIdSql, [newParticipantsHandle], (err, result) => {
            if (err) {
                console.error(`Error getting new participant's user_id`, err);
                return reject(err);
            }

            if (result && result.length > 0 && result[0].user_id) {
                const newParticipantsUserId = result[0].user_id;

                const addParticipantsSql = `
                INSERT INTO ConversationParticipants (conversation_id, user_id) 
                VALUES (?, ?);
            `;

                meNexus.query(addParticipantsSql, [conversation_id, newParticipantsUserId], (err, result) => {
                    if (err) {
                        console.error('Error updating conversation participants', err);
                        return reject(err);
                    }

                    resolve({result});
                })
            } else {
                console.error("No user found with the provided handle")
                return reject(new Error("No user_id found for the provided handle"));
            }
        });
    });
}