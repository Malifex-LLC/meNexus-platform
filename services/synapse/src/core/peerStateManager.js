// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

console.log('Initializing Peer State Manager...');

import {discoveredPeers, connectedPeers} from "./peerState.js";



export const addDiscoveredPeer = (peerId, multiaddrs) => {
    if (!discoveredPeers.has(peerId)) {
        discoveredPeers.set(peerId, { publicKey: null, multiaddrs });
        console.log('Discovered Peers after addDiscoveredPeer:', discoveredPeers);
    }

};

export  const updatePeerPublicKey = (peerId, publicKey) => {
    console.log(`Storing key ${publicKey.slice(0,8)}… for ${peerId}`);
    if (discoveredPeers.has(peerId)) {
        discoveredPeers.get(peerId).publicKey = publicKey;
        console.log('Updated peer publicKey');
        console.log('Discovered Peers @ updatePeerPublicKey: ', discoveredPeers)
    }
};

export const mergeMultiaddrs = (peerId, addrs = []) => {
    if (!discoveredPeers.has(peerId)) {
        discoveredPeers.set(peerId, { publicKey: null, multiaddrs: [] })
    }
    const peer  = discoveredPeers.get(peerId)
    const union = new Set([...peer.multiaddrs, ...addrs.map(a => a.toString())])
    peer.multiaddrs = [...union]
}


export const removeDiscoveredPeer = (peerId) => {
    console.log('DiscoveredPeers @ removeDisocoveredPeer: ', discoveredPeers)
    console.log('Remove discovered peer:', peerId);
    discoveredPeers.delete(peerId);
    connectedPeers.delete(peerId);
};

export const addConnectedPeer = (peerId) => {
    connectedPeers.add(peerId);
};

export const removeConnectedPeer = (peerId) => {
    console.log('Remove connected peer:', peerId);
    connectedPeers.delete(peerId);
};


export const getPeer = (peerId) => discoveredPeers.get(peerId);

export  const getPeerByPublicKey = (publicKey) => {
    console.log('Discovered Peers @ getPeerByPublicKey:', discoveredPeers);
    for (const [peerId, peerData] of discoveredPeers) {
        if (peerData.publicKey === publicKey) {
            return {peerId, ...peerData}; // Return the whole peer object
        }
    }
    return null; // Return null if no peer with that publicKey is found
};

export const isPeerConnected = (peerId) => connectedPeers.has(peerId);

export const getConnectedPeers = () => connectedPeers;

export const getAllDiscoveredPeers = () => discoveredPeers;

export const getAllConnectedPeers = () => new Set(connectedPeers);