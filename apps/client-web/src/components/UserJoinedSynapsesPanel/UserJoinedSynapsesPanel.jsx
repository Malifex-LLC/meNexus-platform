// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import React, {useEffect, useState} from "react";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import SynapseCard from "../SynapseCard/SynapseCard.jsx";
import {Link} from "react-router-dom";

const UserJoinedSynapsesPanel = ({synapses}) => {
    const [synapseMetadataList, setSynapseMetadataList] = useState([]);
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { getSynapseMetadata } = useGetSynapseMetadata();


    useEffect(() => {
        const fetchAllMetadata = async () => {
            const local = await getSynapseMetadata();

            const results = await Promise.all(
                synapses.map(async key => {
                    if (key === local.identity.publicKey) return local;

                    try {
                        const remote = await fetchRemoteSynapseMetadata(key);
                        return remote ?? makeFallback(key);
                    } catch {
                        return makeFallback(key);          // fetch threw → fallback
                    }
                })
            );

            setSynapseMetadataList(results);
        };

        if (synapses.length) fetchAllMetadata();
    }, [synapses]);

    function makeFallback(key) {
        return {
            identity: { publicKey: key, privateKey: "" },
            api: { port: "0000" },
            metadata: {
                name: "Unreachable Synapse",
                description: "Unable to fetch Synapse metadata",
            },
        };
    }

    if (synapseMetadataList.length === 0) {
        return (
            <div className={'flex-1 w-full min-h-0 overflow-y-auto bg-surface/70 border border-border rounded-xl text-foreground'}>
                <div className={'text-3xl text-center'}>
                    Joined Synapses
                </div>
                <div>
                    <SynapseCard
                        description={'Loading Joined Synapses...'}
                    />
                </div>
            </div>
        )
    }

    return (
        <div className={'flex-1 w-full min-h-0 overflow-y-auto bg-surface/70 border border-border rounded-xl text-foreground'}>
            {synapseMetadataList.map((metadata, index) => (
                <Link to={`/synapse/${metadata.identity.publicKey}`} key={index}>
                    <SynapseCard
                        key={index}
                        name={metadata.metadata.name}
                        description={metadata.metadata.description}
                        publicKey={metadata.identity.publicKey}
                    />
                </Link>
            ))}
        </div>
    );
}

export default UserJoinedSynapsesPanel;