// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/**
 * Action types supported by the protocol
 */
export const ACTION_TYPES = {
    // Synchronization Actions
    SYNC: {
        SYNC_STATE: 'SYNC_SYNC_STATE',
        SYNC_USER_DATA: 'SYNC_SYNC_USER_DATA',
    },

    // Data Management Actions
    DATA: {
        QUERY: 'DATA_QUERY',
        AGGREGATE: 'DATA_AGGREGATE',
    },

    // Resource Management Actions
    RESOURCE: {
        CREATE: `RESOURCE_CREATE`,
        FETCH: `RESOURCE_FETCH`,
        UPDATE: `RESOURCE_UPDATE`,
        DELETE: `RESOURCE_DELETE`,
    },

    // Peer Management Actions
    PEER: {
        ADD_PEER: 'PEER_ADD_PEER',
        REMOVE_PEER: 'PEER_REMOVE_PEER',
        REQUEST_PUBLIC_KEY: 'PEER_REQUEST_PUBLIC_KEY',
        RESPONSE_PUBLIC_KEY: 'PEER_RESPONSE_PUBLIC_KEY',
    },

    // Messaging and Notifications
    MSG: {
        SEND_MESSAGE: 'MSG_SEND_MESSAGE',
        NOTIFY_EVENT: 'MSG_NOTIFY_EVENT',
    },

    // Health Action
    HEALTH: {
        PING: 'HEALTH_PING',
        PONG: 'HEALTH_PONG',
        ECHO: 'HEALTH_ECHO',
    },

    // System or Debugging Actions
    SYSTEM: {
        RESTART: 'SYSTEM_RESTART',
        DEBUG_LOG: 'SYSTEM_DEBUG_LOG',
    },
};

/**
 * Validates if a given action type exists in ACTION_TYPES
 * @param {string} actionType - The message type to validate
 * @returns {boolean} - True if valid, false otherwise
 */
export const isValidActionType = (actionType) => {
    return Object.values(ACTION_TYPES)
        .flatMap((namespace) => Object.values(namespace))
        .includes(actionType);
};