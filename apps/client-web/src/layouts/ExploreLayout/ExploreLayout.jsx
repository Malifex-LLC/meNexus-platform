// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useEffect, useState} from "react";
import {useOutletContext} from "react-router-dom";
import DiscoveredSynapsesPanel from "../../components/DiscoveredSynapsesPanel/DiscoveredSynapsesPanel.jsx";
import useGetAllDiscoveredPeers from "../../api/hooks/useGetAllDiscoveredPeers.js";

function useRootContext() {
    return useOutletContext(); // { sessionUser, user, localSynapseMetadata }
}

const ExploreLayout = ({children}) => {
    const { user } = useRootContext();

    const { getAllDiscoveredPeers } = useGetAllDiscoveredPeers();
    const [discoveredPeers, setDiscoveredPeers] = useState(null);

    useEffect(() => {
        const fetchDiscoveredPeers = async () => {
            const response = await getAllDiscoveredPeers();
            setDiscoveredPeers(response);
            console.log("fetchDiscovered Peers response:", response);
        };
        fetchDiscoveredPeers();
    }, []);

    if (!user || !user.publicKey) {
        return <>Loading...</>;
    }
    return (
        <div className={'flex h-[100dvh] bg-background pt-16'}>
            {/*<Header user={user} localSynapseMetadata={localSynapseMetadata}/>*/}
            <div className={`flex flex-col flex-1 w-full h-full p-2 xl:p-4` }>
                <div className={`text-foreground text-5xl xl:text-7xl font-weight-bold mb-12`}>
                    Explore
                </div>
                <div className={`flex w-full h-full max-w-2xl `}>
                    <DiscoveredSynapsesPanel
                        discoveredPeers={discoveredPeers}
                    />
                </div>
                {children}
            </div>
        </div>
    );
}

export default ExploreLayout;