// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";
import path from 'path';
import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const createChatMessage = (publicKey, activeChannel, content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO ChatMessages (public_key, channel, content)
            VALUES (?, ?, ?)
        `;

        meNexus.query(sql, [publicKey, activeChannel, content], (err, result) => {
            if (err) {
                console.error(err)
                return reject(new Error('Database error'));
            }
            resolve(result.insertId); // Return the new post ID
        });
    });
}

export const getChannelChatMessages = async (channel) => {
    return new Promise((resolve, reject) => {
        const query = `
            SELECT *
            FROM ChatMessages
            WHERE ChatMessages.channel = ?
            ORDER BY ChatMessages.created_at DESC
        `;
        meNexus.query(query, channel, async (err, results) => {
            if (err) {
                console.error('Database error in getAllPosts:', err);
                return reject(err)
            }
            try {
                const enrichedPosts = await Promise.all(results.map(async (chat) => {
                    const user = await getUserByPublicKeyFromDB(chat.public_key);
                    const synapseConfig = await loadConfig(CONFIG_FILE)
                    return {
                        ...chat,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown',
                        synapsePublicKey: synapseConfig.identity.publicKey,
                        synapseUrl: synapseConfig.identity.synapseUrl
                    };
                }));
                resolve(enrichedPosts);
            } catch (error) {
                console.error('Error enriching posts:', error);
                reject(error);
            }
        });
    });
};

export default {
    createChatMessage,
    getChannelChatMessages
}