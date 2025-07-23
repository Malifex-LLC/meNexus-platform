// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { SNP_VERSION } from './version.js';
import { MESSAGE_TYPES, isValidMessageType } from './messageTypes.js';
import { ACTION_TYPES, isValidActionType } from './actionTypes.js';
import { v4 as uuidv4 } from 'uuid';
import {isValidResourceType} from "#src/protocols/snp/resourceTypes.js"; // Use UUID for unique request IDs


/**
 * Utility to create a standardized SNP message
 * @param {string} messageType - The type of the message (e.g., SYNCHRONIZE_STATE, REQUEST)
 * @param {string} actionType - The type of action (e.g., DATA_QUERY, RESOURCE_FETCH)
 * @param {object} payload - The data being sent
 * @param {object} meta - Additional metadata (e.g., timestamp)
 * @returns {object} - The constructed message
 */
export const createMessage = (messageType, actionType = {}, resourceType = {}, payload = {}, meta = {}) => {
    //console.log('createMessage: ', messageType, actionType, payload, meta);
    if (!isValidMessageType(messageType)) {
        throw new Error(`Invalid message type: ${messageType}`);
    }
    console.log('Valid message type: ', messageType);

    if (!isValidActionType(actionType)) {
        throw new Error(`Invalid action type: ${actionType}`);
    }
    console.log('Valid action type: ', actionType);

    if (!isValidResourceType(resourceType)) {
        throw new Error(`Invalid resource type: ${resourceType}`);
    }
    console.log('Valid resource type: ', resourceType);

    return {
        protocol: 'SNP',
        version: SNP_VERSION,
        messageType,
        actionType,
        resourceType,
        payload,
        meta: {
            requestId: meta.requestId ?? uuidv4(),
            sender: meta.sender ?? '',
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
    //console.log('encodeMessage called for:', message);
    return JSON.stringify(message) + '\n';
}

/**
 * Utility to decode a JSON string into a message object
 * @param {string} rawMessage - The JSON string to decode
 * @returns {object} - The decoded message
 * @throws {Error} - If the message format is invalid
 */
export const decodeMessage = (rawMessage) => {
    //console.log('decodeMessage called for:', rawMessage);
    try {
        const message = JSON.parse(rawMessage);

        // Validate message structure
        if (
            !message.protocol || message.protocol !== 'SNP' ||
            !message.version ||
            !message.messageType ||
            !message.actionType ||
            !message.payload ||
            !message.meta
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
    //console.log('validateMessage called for:', message);
    if (message.protocol !== 'SNP') {
        throw new Error(`Invalid protocol: ${message.protocol}`);
    }

    if (message.version !== SNP_VERSION) {
        throw new Error(`Unsupported protocol version: ${message.version}`);
    }

    if (!Object.values(MESSAGE_TYPES)
        .flatMap((namespace) => Object.values(namespace))
        .includes(message.messageType)) {
        throw new Error(`Invalid message type: ${message.messageType}`);
    }

    if (!Object.values(ACTION_TYPES)
        .flatMap((namespace) => Object.values(namespace))
        .includes(message.actionType)) {
        throw new Error(`Invalid action type: ${message.actionType}`);
    }

    return true;
};