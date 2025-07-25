// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/**
 * Resource types supported by the protocol
 */
export const RESOURCE_TYPES = {
    SYNAPSE_METADATA: 'SYNAPSE_METADATA',
    PEER_PUBLIC_KEY: 'PEER_PUBLIC_KEY',
    USER: 'USER',
    ALL_USERS: 'ALL_USERS',
    SYNAPSE_POST_BOARDS: 'SYNAPSE_POST_BOARDS',
    POST: 'POST',
    MEDIA: 'MEDIA',
    ALL_POSTS: 'ALL_POSTS',
    BOARD_POSTS: 'BOARD_POSTS',
    COMMENTS: 'COMMENTS',
    CONVERSATION: 'CONVERSATION',
    MESSAGE: 'MESSAGE',
    TOPIC: 'TOPIC',
    NOTIFICATION: 'NOTIFICATION',
    HEALTH_STATUS: 'HEALTH_STATUS',
};

/**
 * Validates if a given resource type exists in RESOURCE_TYPES
 * @param {string} resourceType - The resource type to validate
 * @returns {boolean} - True if valid, false otherwise
 */
export const isValidResourceType = (resourceType) => {
    return Object.values(RESOURCE_TYPES).includes(resourceType);
};