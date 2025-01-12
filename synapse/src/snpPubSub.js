import { EventEmitter } from 'events';
import { createLibp2pInstance } from '../config/libp2p.js'
import { isValidMessageType } from '../../protocols/snp/messageTypes.js'
import { isValidActionType } from '../../protocols/snp/actionTypes.js'
import { isValidResourceType } from '../../protocols/snp/resourceTypes.js'

// SNP message validation function
const validateMessage = (message) => {
    if (!message.type || !message.action || !message.payload) {
        throw new Error('Invalid SNP message: Missing required fields');
    }

    const validMessageType = isValidMessageType(message.type)
    if (!validMessageType) {
        throw new Error('Invalid SNP message type');
    }

    const validActionType = isValidActionType(message.action);
    if (!validActionType) {
        throw new Error('Invalid SNP action type');
    }

    if(message.payload.resource_type) {
        const validResourceType = isValidResourceType(message.payload.resource_type);
        if (!validResourceType) {
            throw new Error('Invalid SNP resource type');
        }
    }
};

export const initializeSnpPubSub = async (externalLibp2pInstance = null) => {
    console.log('Initializing snpPubSub...');
    const libp2pInstance = externalLibp2pInstance || await createLibp2pInstance();

    if (!libp2pInstance) {
        throw new Error('libp2p instance is required to initialize SNPubSub');
    }

    const events = new EventEmitter();

    // Handle incoming messages
    const handleMessage = (event) => {
        try {
            console.log('Event details:', event.detail); // Log the entire event detail

            const topic = event.detail.topic;
            const rawMessageBuffer = event.detail.data; // Use `data` field for the message payload

            // Log raw buffer
            console.log('Raw buffer received:', rawMessageBuffer);

            if (!rawMessageBuffer || rawMessageBuffer.length === 0) {
                throw new Error('Received empty message buffer');
            }

            const rawMessage = new TextDecoder().decode(rawMessageBuffer);

            // Log decoded string
            console.log('Decoded message string:', rawMessage);

            const message = JSON.parse(rawMessage); // Attempt to parse JSON

            validateMessage(message);
            events.emit(topic, message);
            console.log(`Message received on topic ${topic}:`, message);
        } catch (error) {
            console.error('Failed to handle incoming message:', error);
        }
    };

    // Attach the handler to the libp2p pubsub events
    libp2pInstance.services.pubsub.addEventListener('message', handleMessage);

    // Publish a message to a topic
    const publish = async (topic, message) => {
        try {
            validateMessage(message);
            const messageString = JSON.stringify(message);
            const encodedMessage = new TextEncoder().encode(messageString);

            console.log('Publishing message:', messageString);
            console.log('Encoded message buffer:', encodedMessage);

            await libp2pInstance.services.pubsub.publish(topic, encodedMessage);
            console.log(`Published to topic: ${topic}`);
        } catch (error) {
            console.error(`Failed to publish message to ${topic}:`, error);
        }
    };

    // Subscribe to a topic
    const subscribe = (topic, callback) => {
        try {
            libp2pInstance.services.pubsub.subscribe(topic);
            events.on(topic, callback);
            console.log(`Subscribed to topic: ${topic}`);
        } catch (error) {
            console.error(`Failed to subscribe to ${topic}:`, error);
        }
    };

    // Unsubscribe from a topic
    const unsubscribe = (topic) => {
        try {
            libp2pInstance.services.pubsub.unsubscribe(topic);
            events.removeAllListeners(topic);
            console.log(`Unsubscribed from topic: ${topic}`);
        } catch (error) {
            console.error(`Failed to unsubscribe from ${topic}:`, error);
        }
    };

    return { publish, subscribe, unsubscribe };
};