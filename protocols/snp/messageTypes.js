/**
 * Message types supported by the protocol
 */
export const MESSAGE_TYPES = {
    // Synchronization messages: Manage state synchronization between Synapses
    SYNC: {
        SYNCHRONIZE_STATE: 'SYNC_SYNCHRONIZE_STATE',
        STATE_UPDATE: 'SYNC_STATE_UPDATE',
    },

    // Data messages: Handle requests and responses for data transfer
    DATA: {
        REQUEST: 'DATA_REQUEST',
        RESPONSE: 'DATA_RESPONSE',
        ERROR: 'DATA_ERROR',
    },

    // Peer management messages: Manage peer connections and discovery
    PEER: {
        ANNOUNCE: 'PEER_ANNOUNCE',
        DISCONNECT: 'PEER_DISCONNECT',
        REQUEST_PUBLIC_KEY: 'PEER_REQUEST_PUBLIC_KEY',
        RESPONSE_PUBLIC_KEY: 'PEER_RESPONSE_PUBLIC_KEY',
    },

    // Messaging: Direct or broadcast messages between peers
    MSG: {
        BROADCAST: 'MSG_BROADCAST',
        DIRECT_MESSAGE: 'MSG_DIRECT_MESSAGE',
    },

    // Health checks: Monitor and debug Synapse health
    HEALTH: {
        PING: 'HEALTH_PING',
        PONG: 'HEALTH_PONG',
        ECHO: 'HEALTH_ECHO',
        CHECK: 'HEALTH_CHECK',
        STATUS: 'HEALTH_STATUS',
    },

    // Actions and events: Trigger actions or emit events between Synapses
    ACTION: {
        TRIGGER: 'ACTION_TRIGGER',
        EMIT: 'ACTION_EMIT',
    },
};

/**
 * Validates if a given message type exists in MESSAGE_TYPES
 * @param {string} messageType - The message type to validate
 * @returns {boolean} - True if valid, false otherwise
 */
export const isValidMessageType = (messageType) => {
    return Object.values(MESSAGE_TYPES)
        .flatMap((namespace) => Object.values(namespace))
        .includes(messageType);
};