// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import { getUserByPublicKeyFromDB, getUserByHandleFromDB } from "#src/orbitdb/globalUsers.js";

export const getConversations = async (userPublicKey) => {
    try {
        const sql = `
            SELECT
                c.conversation_id,
                cp2.public_key AS participant_id,
                EXISTS (
                    SELECT 1
                    FROM Messages m
                    WHERE m.conversation_id = c.conversation_id AND m.is_read = FALSE AND m.receiver_public_key = ?
                ) AS has_unread_messages
            FROM Conversations c
                JOIN ConversationParticipants cp1 ON c.conversation_id = cp1.conversation_id
                JOIN ConversationParticipants cp2 ON c.conversation_id = cp2.conversation_id
            WHERE cp1.public_key = ?
              AND cp2.public_key != ?
        `;

        const results = await new Promise((resolve, reject) => {
            meNexus.query(sql, [userPublicKey, userPublicKey, userPublicKey], (err, rows) => {
                if (err) {
                    console.error('Error fetching conversations:', err);
                    return reject(err);
                }
                resolve(rows);
            });
        });

        // Enrich each result with globalUsers data
        const enrichedResults = await Promise.all(
            results.map(async (row) => {
                const participant = await getUserByPublicKeyFromDB(row.participant_id);
                return {
                    ...row,
                    participant_handle: participant?.handle || null,
                    participant_profileName: participant?.profileName || null,
                    participant_profilePicture: participant?.profilePicture || null,
                };
            })
        );

        console.log('/getConversations enriched:', enrichedResults);
        return enrichedResults;

    } catch (err) {
        console.error("Error in getConversations:", err);
        throw err;
    }
};

export const createConversation = (publicKey) => {
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
                    ConversationParticipants (conversation_id, public_key) 
                VALUES (?, ?);
            `;

            meNexus.query(addSenderParticipantSql, [newConversationId, publicKey], (err, results) => {
                if (err) {
                    console.error('Error adding sender to  ConversationParticipants', err);
                    return reject(err);
                }

                resolve({conversation_id: newConversationId})
            });
        });
    });
}

export const updateConversationParticipants = async (conversation_id, newParticipantsHandle) => {
    try {
        const user = await getUserByHandleFromDB(newParticipantsHandle);
        const participantPublicKey = user.publicKey;

        const addParticipantsSql = `
                    INSERT INTO ConversationParticipants (conversation_id, public_key)
                    VALUES (?, ?);
                `;
        const result = await new Promise((resolve, reject) => {
            meNexus.query(addParticipantsSql, [conversation_id, participantPublicKey], (err, result) => {
                if (err) {
                    console.error('Error updating conversation participants', err);
                    return reject(err);
                }
                resolve({result});
            })
        })
    } catch (err) {
        console.error("Error updating conversation participants", err);
        throw err;
    }

}

export default {
    getConversations,
    createConversation,
    updateConversationParticipants,
}