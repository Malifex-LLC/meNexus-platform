// Import shared peer state
//import peerState from "#src/peerState.js";
console.log("Loading PeerStateManagerSingleton...");
console.trace();
console.log("Resolved path for PeerStateManagerSingleton:", import.meta.url);

const PeerStateManagerSingleton = (function () {
    let instance;
    let peerState = null; // Shared state

    // Constructor for the singleton
    function PeerStateManagerSingleton() {
        if (instance) {
            throw new Error("PeerStateManagerSingleton instance already exists. Use getInstance method.");
        }
        console.log("Initializing PeerStateManagerSingleton...");
        peerState = {
            discoveredPeers: new Map(),
            connectedPeers: new Set(),
        };

    }

    // Static method to get the singleton instance
    PeerStateManagerSingleton.getInstance = function () {
        if (!instance) {
            instance = new PeerStateManagerSingleton();
        }
        return instance;
    };

    // Prevent cloning
    PeerStateManagerSingleton.prototype.clone = function () {
        throw new Error("Cloning of singleton instance is not allowed.");
    };

    // Prevent custom deserialization
    PeerStateManagerSingleton.prototype.customDeserialize = function () {
        throw new Error("Deserialization of singleton instance is not allowed.");
    };

    // Prevent deserialization via JSON.parse
    PeerStateManagerSingleton.revive = function (key, value) {
        if (key === "" && value && value.__isSingleton) {
            return instance;
        }
        return value;
    };

    // Custom serialization behavior
    PeerStateManagerSingleton.prototype.toJSON = function () {
        return JSON.stringify({ __isSingleton: true, peerState: peerState });
    };

    // Define methods for managing peer state
    PeerStateManagerSingleton.prototype.addDiscoveredPeer = function (peerId, multiaddrs) {
        if (!peerState.discoveredPeers.has(peerId)) {
            peerState.discoveredPeers.set(peerId, { publicKey: null, multiaddrs });
            console.log("Discovered Peers after addDiscoveredPeer:", peerState.discoveredPeers);
        }
    };

    PeerStateManagerSingleton.prototype.updatePeerPublicKey = function (peerId, publicKey) {
        if (peerState.discoveredPeers.has(peerId)) {
            peerState.discoveredPeers.get(peerId).publicKey = publicKey;
            console.log("Updated peer publicKey");
            console.log("Discovered Peers @ updatePeerPublicKey: ", peerState.discoveredPeers);
        }
    };

    PeerStateManagerSingleton.prototype.removeDiscoveredPeer = function (peerId) {
        console.log("DiscoveredPeers @ removeDiscoveredPeer:", peerState.discoveredPeers);
        console.log("Removing discovered peer:", peerId);
        peerState.discoveredPeers.delete(peerId);
        peerState.connectedPeers.delete(peerId);
    };

    PeerStateManagerSingleton.prototype.addConnectedPeer = function (peerId) {
        peerState.connectedPeers.add(peerId);
    };

    PeerStateManagerSingleton.prototype.removeConnectedPeer = function (peerId) {
        console.log("Removing connected peer:", peerId);
        peerState.connectedPeers.delete(peerId);
    };

    PeerStateManagerSingleton.prototype.getPeer = function (peerId) {
        return peerState.discoveredPeers.get(peerId);
    };

    PeerStateManagerSingleton.prototype.getPeerByPublicKey = function (publicKey) {
        console.log("Discovered Peers @ getPeerByPublicKey:", peerState.discoveredPeers);
        for (const [peerId, peerData] of peerState.discoveredPeers.entries()) {
            if (peerData.publicKey === publicKey) {
                return { peerId, ...peerData };
            }
        }
        return null;
    };

    PeerStateManagerSingleton.prototype.isPeerConnected = function (peerId) {
        return peerState.connectedPeers.has(peerId);
    };

    PeerStateManagerSingleton.prototype.getConnectedPeers = function () {
        return peerState.connectedPeers;
    };

    PeerStateManagerSingleton.prototype.getAllDiscoveredPeers = function () {
        return peerState.discoveredPeers;
    };

    PeerStateManagerSingleton.prototype.getAllConnectedPeers = function () {
        return new Set(peerState.connectedPeers);
    };

    return PeerStateManagerSingleton;
})();

export default PeerStateManagerSingleton;
