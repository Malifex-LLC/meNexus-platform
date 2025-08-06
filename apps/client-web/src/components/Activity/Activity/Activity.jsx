// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { Link } from "react-router-dom";
import { useEffect, useState } from "react";
import useGetUser from "../../../api/hooks/useGetUser.js";
import useGetSynapseMetadata from "../../../api/hooks/useGetSynapseMetadata.js";
import useFetchRemoteSynapseMetadata from "../../../api/hooks/useFetchRemoteSynapseMetadata.js";

/* ---------------------------------------------------------------- helpers */



const makeDescription = ({
                             activity,
                             actor,
                             targetUser = null,
                             targetSynapse = null,
                             mode
}) => {

    switch (activity.type) {
        case 'POSTED':
            return (
                <>
                    {mode === 'SYNAPSE' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> created a new
                            <Link to={`/post/${activity.object_id}`}><span className={'text-accent cursor-pointer'}> post</span></Link>
                        </>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> created a
                            <Link to={`/post/${activity.object_id}`}><span className={'text-neutral cursor-pointer'}> post</span></Link>
                            <> in </>
                            <Link to={`/synapse/${activity.context_id}`}><span className={'text-accent cursor-pointer'}> {targetSynapse.metadata.name}</span></Link>
                        </>
                    )}
                </>
            );

        case 'COMMENTED':
            return (
                <>
                    {mode === 'SYNAPSE' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> commented on a
                            <Link to={`/post/${activity.object_id}`}><span className={'text-accent cursor-pointer'}> post</span></Link>
                        </>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> commented on a
                            <Link to={`/post/${activity.object_id}`}><span className={'text-neutral cursor-pointer'}> post</span></Link>
                            <> in </>
                            <Link to={`/synapse/${activity.context_id}`}><span className={'text-accent cursor-pointer'}> {targetSynapse.metadata.name}</span></Link>
                        </>
                    )}
                </>
            );

        case 'FOLLOWED':
            return (
                <>
                    <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> followed <Link to={`/profile/${targetUser.handle}`}><span className={'text-brand cursor-pointer'}>@{targetUser.handle}</span></Link>
                </>
            );

        case 'UNFOLLOWED':
            return (
                <>
                    <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> unfollowed <Link to={`/profile/${targetUser.handle}`}><span className={'text-brand cursor-pointer'}>@{targetUser.handle}</span></Link>
                </>
            );

        case 'JOINED':
            return (
                <>
                    {mode === 'SYNAPSE' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> joined!
                        </>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> joined
                            <Link to={`/synapse/${activity.context_id}`}><span className={'text-accent cursor-pointer'}> {targetSynapse.metadata.name}</span></Link>
                        </>
                    )}
                </>
            );

        case 'LEFT':
            return (
                <>
                    {mode === 'SYNAPSE' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> left!
                        </>
                    )}
                    {mode === 'GLOBAL' && (
                        <>
                            <Link to={`/profile/${actor.handle}`}><span className={'text-brand cursor-pointer'}>@{actor.handle}</span></Link> left
                            <Link to={`/synapse/${activity.context_id}`}><span className={'text-accent cursor-pointer'}> {targetSynapse.metadata.name}</span></Link>
                        </>
                    )}
                </>
            );

        default:
            return (
                <>
                    {actor.handle} did something
                </>
            );
    }
}

/* -------------------------------------------------------------- component */
const Activity = ({ activity, mode }) => {
    const [isLocalSynapse, setIsLocalSynapse] = useState(true);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [user, setUser] = useState(null);
    const [description, setDescription] = useState(null);
    const { getUser } = useGetUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();



    useEffect(() => {
        const fetchUserData = async () => {
            try {
                const userData = await getUser(activity.actor_public_key);
                setUser(userData);
            } catch (error) {
                console.error("Error fetching userData: ", error);
            }
        }
        fetchUserData();
    }, [activity.actor_public_key])

    useEffect(() => {
        const createDescription = async () => {
            if (!user) return;

            switch (activity.type) {
                case 'FOLLOWED':
                case 'UNFOLLOWED': {
                    const targetUser = await getUser(activity.target_id);
                    const userDescription = makeDescription({
                        activity,
                        actor: user,
                        targetUser,
                        mode,
                    });
                    setDescription(userDescription);
                    break;
                }

                case 'JOINED':
                case 'LEFT':
                case 'POSTED':
                case 'COMMENTED': {
                    await fetchContextSynapse();
                    break;
                }

                default: {
                    const fallbackDescription = makeDescription({ activity, actor: user, mode });
                    setDescription(fallbackDescription);
                    break;
                }
            }
        };

        const fetchContextSynapse = async () => {
            try {
                if (activity.context_type !== 'SYNAPSE') return;

                const localSynapse = await getSynapseMetadata();
                const isLocal = localSynapse.identity.publicKey === activity.context_id;

                const contextSynapse = isLocal
                    ? localSynapse
                    : await fetchRemoteSynapseMetadata(activity.context_id);

                setSynapseMetadata(contextSynapse);
                setIsLocalSynapse(isLocal);

                const contextDescription = makeDescription({
                    activity,
                    actor: user,
                    targetSynapse: contextSynapse,
                    mode,
                });

                setDescription(contextDescription);
            } catch (err) {
                console.error("Error fetching context Synapse metadata:", err);
            }
        };

        createDescription();
    }, [user]);



    const date = new Date(activity.published);

    if (!description) {
        return <div>Loading...</div>;
    }


    return (
        <div className="flex flex-col flex-1 p-4 py-5 rounded-xl text-xl md:text-xs xl:text-md 2xl:text-xl
                    bg-surface/40 text-foreground shadow-2xl"
        >
            {description}
            <span className="block text-xs opacity-60 mt-1">
                {date.toLocaleString()}
            </span>
        </div>
    );
};

export default Activity;
