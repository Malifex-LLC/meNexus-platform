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
    },

    // Messaging and Notifications
    MSG: {
        SEND_MESSAGE: 'MSG_SEND_MESSAGE',
        NOTIFY_EVENT: 'MSG_NOTIFY_EVENT',
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