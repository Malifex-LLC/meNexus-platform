/**
 * Resource types supported by the protocol
 */
export const RESOURCE_TYPES = {
    USER: 'USER',
    POST: 'POST',
    COMMENT: 'COMMENT',
    CONVERSATION: 'CONVERSATION',
    MESSAGE: 'MESSAGE',
    TOPIC: 'TOPIC',
    NOTIFICATION: 'NOTIFICATION',
};

/**
 * Validates if a given resource type exists in RESOURCE_TYPES
 * @param {string} resourceType - The resource type to validate
 * @returns {boolean} - True if valid, false otherwise
 */
export const isValidResourceType = (resourceType) => {
    return Object.values(RESOURCE_TYPES).includes(resourceType);
};