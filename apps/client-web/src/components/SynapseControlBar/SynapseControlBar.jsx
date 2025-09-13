// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {Link, useLocation, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import { IoMdArrowDropdown } from "react-icons/io";
import JoinedSynapsesTray from "../JoinedSynapsesTray/JoinedSynapsesTray.jsx";
import synapse from "../../pages/Synapse/Synapse.jsx";
import useJoinSynapse from "../../api/hooks/useJoinSynapse.js";
import useLeaveSynapse from "../../api/hooks/useLeaveSynapse.js";
import useJoinRemoteSynapse from "../../api/hooks/useJoinRemoteSynapse.js";
import useLeaveRemoteSynapse from "../../api/hooks/useLeaveRemoteSynapse.js";


const SynapseControlBar = ({synapses = [], publicKey}) => {
    const location = useLocation();
    const isActive = (path) => location.pathname.startsWith(path) ? "text-brand" : "text-foreground";
    const {synapsePublicKey} = useParams();
    const [isLocalSynapse, setIsLocalSynapse] = useState(null);
    const [synapseMetadataList, setSynapseMetadataList] = useState([]);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [showJoinedSynapsesTray, setShowJoinedSynapsesTray] = useState(false);
    const [currentSynapseMetadata, setCurrentSynapseMetadata] = useState(null);
    const [isSynapseMember, setIsSynapseMember] = useState(false);
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { joinSynapse } = useJoinSynapse();
    const { leaveSynapse } = useLeaveSynapse();
    const { joinRemoteSynapse } = useJoinRemoteSynapse();
    const { leaveRemoteSynapse } = useLeaveRemoteSynapse();

    useEffect(() => {
        const fetchCurrentSynapseMetadata = async () => {
            const localSynapseMetadata = await getSynapseMetadata();
            if (synapsePublicKey === localSynapseMetadata.identity.publicKey) {
                setCurrentSynapseMetadata(localSynapseMetadata);
                setIsLocalSynapse(true)
            } else {
                const response = await fetchRemoteSynapseMetadata(synapsePublicKey);
                setCurrentSynapseMetadata(response);
                setIsLocalSynapse(false)
            }

            const isMember = synapses.includes(synapsePublicKey);
            setIsSynapseMember(isMember);
        }
        fetchCurrentSynapseMetadata();
    }, [synapsePublicKey, synapses])

    useEffect(() => {
        const fetchAllMetadata = async () => {
            const localSynapseMetadata = await getSynapseMetadata();
            const results = await Promise.all(
                synapses.map(async (synapse) => {
                    if (synapse === localSynapseMetadata.identity.publicKey) {
                        if (localSynapseMetadata.identity.publicKey === synapsePublicKey) {
                            setSynapseMetadata(localSynapseMetadata);
                        }
                        return localSynapseMetadata;
                    }
                    const fetchedSynapseMetadata = await fetchRemoteSynapseMetadata(synapse);
                    if(fetchedSynapseMetadata.identity.publicKey === synapsePublicKey) {
                        setSynapseMetadata(fetchedSynapseMetadata);
                        return fetchedSynapseMetadata;
                    }
                    return fetchedSynapseMetadata;
                })
            );
            setSynapseMetadataList(results);
        };

        if (synapses.length > 0) {
            fetchAllMetadata();
        }
    }, [synapsePublicKey, synapses]);

    const toggleJoinedSynapsesTray = () => {
        setShowJoinedSynapsesTray((prevState) => !prevState);
    }

    const handleJoin = async () => {
        console.log("isLocalSynapse:", isLocalSynapse);
        if (isLocalSynapse) {
            await joinSynapse(publicKey);
        } else {
            await joinRemoteSynapse(publicKey, synapsePublicKey);
        }
    }

    const handleLeave = async () => {
        console.log("isLocalSynapse:", isLocalSynapse);
        if (isLocalSynapse) {
            await leaveSynapse(publicKey);
        } else {
            await leaveRemoteSynapse(publicKey, synapsePublicKey);
        }
    }

    return (

        <div className="flex flex-col py-2 px-4 rounded-xl bg-surface">
            <div>
                {currentSynapseMetadata && synapseMetadataList ? (
                    <div className="flex flex-row pt-4 text-foreground">
                        <div>
                            <h1 className="text-3xl text-brand font-bold">{currentSynapseMetadata.metadata.name}</h1>
                            <p className="text-lg text-foreground">
                                {currentSynapseMetadata.metadata.description}
                            </p>

                            {isSynapseMember ? (
                                <button
                                    className={`px-1 mt-4 text-foreground bg-background rounded-xl hover:bg-brand/60 hover:text-foreground-alt hover:cursor-pointer`}
                                    onClick={async () => {
                                        await handleLeave();
                                        setIsSynapseMember(false);
                                    }}
                                >
                                    Leave Synapse
                                </button>
                            ) : (
                                <button
                                    className={`px-1 mt-4 text-foreground-alt bg-brand rounded-xl hover:bg-primary hover:cursor-pointer`}
                                    onClick={async () => {
                                        await handleJoin();
                                        setIsSynapseMember(true);
                                    }}
                                >
                                    Join Synapse
                                </button>
                            )}
                        </div>
                        <div className={'flex flex-col'}>
                            <div
                                className={'flex text-3xl'}
                                onClick={toggleJoinedSynapsesTray}
                            >
                                <div className={'flex h-full text-5xl cursor-pointer mt-16 hover:text-brand/60'}>
                                    <IoMdArrowDropdown />
                                </div>
                            </div>
                            <div>
                                {showJoinedSynapsesTray && (
                                    <div className="absolute  z-50 bg-header-bg/70 backdrop-blur-xs rounded-2xl shadow-2xl w-2xl">
                                        <JoinedSynapsesTray
                                            synapseMetadataList={synapseMetadataList}
                                        />
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                ) : (
                    <div className="p-4">Loading synapse info…</div>
                )}
            </div>
        </div>
    )
}

export default SynapseControlBar;