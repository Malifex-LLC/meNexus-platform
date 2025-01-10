import { SNP_VERSION } from './version.js';
import { MESSAGE_TYPES, isValidMessageType } from './messageTypes.js';

/**
 * Utility to create a standardized SNP message
 * @param {string} type - The type of the message (e.g., SYNCHRONIZE_STATE, REQUEST)
 * @param {object} payload - The data being sent
 * @param {object} meta - Additional metadata (e.g., timestamp)
 * @returns {object} - The constructed message
 */
export const createMessage = (type, payload = {}, meta = {}) => {
    console.log('createMessage: ', type, payload, meta);
    if (!isValidMessageType(type)) {
        throw new Error(`Invalid message type: ${type}`);
    }
    console.log('Validate message type: ', type);

    return {
        protocol: 'SNP',
        version: SNP_VERSION,
        type,
        payload,
        meta: {
            sender: meta.sender,
            timestamp: meta.timestamp || new Date().toISOString()
        },
    };
};

/**
 * Utility to encode a message into a JSON string
 * @param {object} message - The message to encode
 * @returns {string} - JSON string
 */
export const encodeMessage = (message) => {
    console.log('encodeMessage called for:', message);
    JSON.stringify(message);
}

/**
 * Utility to decode a JSON string into a message object
 * @param {string} rawMessage - The JSON string to decode
 * @returns {object} - The decoded message
 * @throws {Error} - If the message format is invalid
 */
export const decodeMessage = (rawMessage) => {
    console.log('decodeMessage called for:', rawMessage);
    try {
        const message = JSON.parse(rawMessage);

        // Validate message structure
        if (
            !message.protocol ||
            message.protocol !== 'SNP' ||
            !message.version ||
            !message.type ||
            !message.payload
        ) {
            throw new Error('Invalid SNP message format');
        }

        return message;
    } catch (error) {
        throw new Error('Failed to decode message: ' + error.message);
    }
};

/**
 * Validate a decoded message object
 * @param {object} message - The message to validate
 * @returns {boolean} - True if valid, throws an error otherwise
 */
export const validateMessage = (message) => {
    console.log('validateMessage called for:', message);
    if (message.protocol !== 'SNP') {
        throw new Error(`Invalid protocol: ${message.protocol}`);
    }

    if (message.version !== SNP_VERSION) {
        throw new Error(`Unsupported protocol version: ${message.version}`);
    }

    if (!Object.values(MESSAGE_TYPES).includes(message.type)) {
        throw new Error(`Invalid message type: ${message.type}`);
    }

    return true;
};