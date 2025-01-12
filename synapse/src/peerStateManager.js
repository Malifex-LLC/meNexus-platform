import {discoveredPeers, connectedPeers} from "./peerState.js";

export const addDiscoveredPeer = async (peerId, multiaddrs) => {
    if (!discoveredPeers.has(peerId)) {
        discoveredPeers.set(peerId, { publicKey: null, multiaddrs });
        console.log('Discovered Peers after addDiscoveredPeer:', discoveredPeers);
    }

};

export  const updatePeerPublicKey = async (peerId, publicKey) => {
    if (discoveredPeers.has(peerId)) {
        discoveredPeers.get(peerId).publicKey = publicKey;
        console.log('Updated peer publicKey');
        console.log('Discovered Peers @ updatePeerPublicKey: ', discoveredPeers)
    }
};

export const removeDiscoveredPeer = async (peerId) => {
    discoveredPeers.delete(peerId);
    connectedPeers.delete(peerId);
};

export const addConnectedPeer = async (peerId) => {
    connectedPeers.add(peerId);
};

export const removeConnectedPeer = async (peerId) => {
    connectedPeers.delete(peerId);
};


export const getPeer = async (peerId) => discoveredPeers.get(peerId);

export  const getPeerByPublicKey = async (publicKey) => {
    console.log('Discovered Peers @ getPeerByPublicKey:', discoveredPeers);
    for (const [peerId, peerData] of discoveredPeers) {
        if (peerData.publicKey === publicKey) {
            return {peerId, ...peerData}; // Return the whole peer object
        }
    }
    return null; // Return null if no peer with that publicKey is found
};

export const isPeerConnected = async (peerId) => connectedPeers.has(peerId);

export const getConnectedPeers = async () => connectedPeers;

export const getAllDiscoveredPeers = async () => discoveredPeers;

export const getAllConnectedPeers = async () => new Set(connectedPeers);