// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate, useParams} from "react-router-dom";
import DiscoveredSynapsesPanel from "../../components/DiscoveredSynapsesPanel/DiscoveredSynapsesPanel.jsx";
import useGetAllDiscoveredPeers from "../../api/hooks/useGetAllDiscoveredPeers.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";


const ExploreLayout = ({children}) => {
    const navigate = useNavigate(); // React Router navigate
    const { synapsePublicKey } = useParams(); // Extract synapsePublicKey from the URL (if available)

    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const [localSynapseMetadata, setLocalSynapseMetadata] = useState(null);

    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();

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


    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                console.log("Fetching current user session...");
                const response = await getSessionUser();
                setSessionUser(response.data)
            } catch (error) {
                console.error("Error fetching current session user:", error);
                navigate('/login');
            }
        }
        fetchSessionUser();
    }, [])

    useEffect(() => {
        const fetchUser = async () => {
            try {
                const response = await getUser(sessionUser.publicKey);
                setUser(response);
            } catch (error) {
                console.error("Error fetching current user:", error);
            }
        }
        fetchUser();
    }, [sessionUser])

    useEffect(() => {
        const fetchSynapseMetadata = async () => {
            const localSynapseData = await getSynapseMetadata();
            setLocalSynapseMetadata(localSynapseData)
        };
        fetchSynapseMetadata();
    },[synapsePublicKey]);

    if (!user || !user.publicKey) {
        return <>Loading...</>;
    }
    return (
        <div className={'flex h-[100dvh] pt-17 bg-background'}>
            <Header
                user={user}
                localSynapseMetadata={localSynapseMetadata}
            />
            <div className={`flex flex-col flex-1 w-full h-full p-2 xl:p-4 `}>
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