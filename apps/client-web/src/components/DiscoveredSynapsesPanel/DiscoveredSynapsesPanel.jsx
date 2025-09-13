import { useEffect, useState } from "react";
import {Link} from 'react-router-dom'
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import SynapseCard from "../SynapseCard/SynapseCard.jsx";

const DiscoveredSynapsesPanel = ({discoveredPeers}) => {
    const {fetchRemoteSynapseMetadata} = useFetchRemoteSynapseMetadata();
    const [synapseMetadata, setSynapseMetadata] = useState();

    useEffect(() => {
        const fetchMetadata = async () => {
            if (discoveredPeers && Object.keys(discoveredPeers).length > 0) {
                const results = await Promise.all(
                    Object.entries(discoveredPeers)
                        .filter(([_, peer]) => peer.publicKey) // skip peers with null keys
                        .map(async ([peerId, peer]) => {
                            try {
                                const response = await fetchRemoteSynapseMetadata(peer.publicKey);
                                return { peerId, metadata: response };
                            } catch (err) {
                                console.error(`Failed to fetch metadata for ${peerId}`, err);
                                return null;
                            }
                        })
                );
                console.log("fetchMetadata results: ", results.filter(Boolean));

                // Filter out any null responses from failed requests
                setSynapseMetadata(results.filter(Boolean));
            }
        };

        fetchMetadata();
    }, [discoveredPeers]);



    if (!discoveredPeers) {
        return (<div>Loading Discovered Synapses</div>)
    }
    return (
        <div className={`flex flex-col flex-1 h-full bg-surface/70 border border-border rounded-xl shadow-xl`}>
            <div className={`p-4 text-3xl bg-surface text-brand text-center border-b border-border rounded-t-xl`}>
                Discovered Synapses
            </div>
            {synapseMetadata ? (
                synapseMetadata.map((synapse, index) => {
                    return (
                        <Link
                            key={index}
                            to={`/synapse/${synapse.metadata.identity.publicKey}`}>
                            <SynapseCard
                                key={index}
                                name={synapse.metadata.metadata.name}
                                description={synapse.metadata.metadata.description}
                                publicKey={synapse.metadata.identity.publicKey}
                            />
                        </Link>




                    )
                })
            ) : (
                <div>Loading Discovered Peers...</div>
            )}

        </div>



    );
};

export default DiscoveredSynapsesPanel;
