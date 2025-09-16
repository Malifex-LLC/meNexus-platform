// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useRef, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useActivityWebSocket from "../../api/hooks/useActivityWebSocket.js";
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import {useParams} from "react-router-dom";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";

const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

const  SynapseActivityPanel = ({isLocalSynapse, synapseMetadata, publicKey, user}) => {
    const { synapsePublicKey } = useParams();
    const [activities, setActivities] = useState([]);
    const [bufferedActivities, setBufferedActivities] = useState([]);

    const synapseCacheRef = useRef(new Map()); // key: publicKey, val: metadata
    const userCacheRef    = useRef(new Map()); // key: publicKey, val: user

    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { getUser } = useGetUser();

    useActivityWebSocket({
        wsUrl: synapseMetadata.identity.webSocketUrl,
        publicKey,
        onActivity: (newActivity) => {
            const parsedDate = new Date(newActivity.published.replace(' ', 'T') + 'Z');
            const safeActivity = {
                ...newActivity,
                published: parsedDate.toISOString(), // ensures consistency
            };

            setBufferedActivities(prev => [safeActivity, ...prev]);
        }
    });


    // Aggregate activities from all followed synapses
    useEffect(() => {
        const getActivities = async () => {
            if (isLocalSynapse) {
                const activitiesData = await getAllActivities();
                setActivities(activitiesData)
            } else {
                const activitiesData = await fetchRemoteSynapseAllActivities(synapsePublicKey);
                setActivities(activitiesData)
            }
        }
        getActivities();
    }, [isLocalSynapse, synapsePublicKey])

    // Once activities change, ensure caches have needed Synapse + User data
    useEffect(() => {
        if (!activities.length) return;

        const fillCaches = async () => {
            // Preload LOCAL synapse metadata into cache (so we never re-fetch it)
            const localSynapseMetadata = await getSynapseMetadata();
            synapseCacheRef.current.set(localSynapseMetadata.identity.publicKey, localSynapseMetadata);

            // ----- SYNAPSES -----
            const synapsePublicKeys = new Set(
                activities
                    .filter(a => a.context_type === 'SYNAPSE' && typeof a.context_id === 'string')
                    .map(a => a.context_id)
            );

            const missingSynapseIds = [...synapsePublicKeys].filter(id => !synapseCacheRef.current.has(id));
            await Promise.all(missingSynapseIds.map(async (id) => {
                try {
                    // if it's local, skip (already set above)
                    if (id === localSynapseMetadata.identity.publicKey) return;
                    const remoteSynapseMetadata = await fetchRemoteSynapseMetadata(id);
                    if (remoteSynapseMetadata) synapseCacheRef.current.set(id, remoteSynapseMetadata);
                } catch (e) {
                    console.warn('Synapse metadata fetch failed for', id, e);
                }
            }));

            // ----- USERS -----
            const actorIds = activities.map(a => a.actor_public_key);
            const followTargetIds = activities
                .filter(a => a.type === 'FOLLOWED' || a.type === 'UNFOLLOWED')
                .map(a => a.target_id);

            const allUserIds = new Set([...actorIds, ...followTargetIds].filter(Boolean));
            const missingUserIds = [...allUserIds].filter(id => !userCacheRef.current.has(id));

            await Promise.all(missingUserIds.map(async (id) => {
                try {
                    const u = await getUser(id);
                    if (u) userCacheRef.current.set(id, u);
                } catch (e) {
                    console.warn('User fetch failed for', id, e);
                }
            }));
        };

        fillCaches();
    }, [activities]);

    const reloadActivities = async () => {
        if (isLocalSynapse) {
            const activitiesData = await getAllActivities();
            setActivities(activitiesData)
            setBufferedActivities([])
        } else {
            const activitiesData = await fetchRemoteSynapseAllActivities(synapsePublicKey);
            setActivities(activitiesData)
            setBufferedActivities([])
        }
    }


    /* group by day */
    const groups = useMemo(() => {
        if (!activities) return {};

        const sorted = [...activities].sort((a, b) => new Date(b.published) - new Date(a.published));

        return sorted.reduce((acc, a) => {
            const key = yyyymmdd(a.published);
            (acc[key] ||= []).push(a);
            return acc;
        }, {});
    }, [activities]);



    if (!activities) {
        return <div>Loading activities...</div>
    }
    return (
        <div className="flex flex-1  overflow-y-auto rounded-xl  text-foreground">

            {/* master timeline line */}
            <ul className="flex flex-col w-full p-4">
                {bufferedActivities.length > 0 && (
                    <div className="text-center p-4 m-4 pl-8">
                        <button
                            onClick={reloadActivities}
                            className="px-4 py-2 bg-brand text-foreground rounded-xl shadow hover:brightness-110"
                        >
                            Load {bufferedActivities.length} new {bufferedActivities.length === 1 ? 'activity' : 'activities'}
                        </button>
                    </div>
                )}
                {Object.entries(groups).map(([day, items]) => (
                    <li key={day} className="flex flex-col w-full">
                        {/* day badge */}
                        <div className="">
                            <span className="px-2 py-0.5 bg-background font-medium text-3xl text-neutral shadow-2xl rounded-xl"
                            >
                                {dayLabel(items[0].published)}
                            </span>
                        </div>

                        {/* items for that day */}
                        <ul className={'flex flex-col w-full py-2 '}>
                            {items.map((activity) => (
                                <li key={activity.id} className="relative flex w-full pl-8 py-16 ">
                                    {/* dot */}
                                    <span className="absolute left-3 top-6 w-3 h-3 rounded-full bg-brand" />
                                    <Activity
                                        activity={activity}
                                        mode="SYNAPSE"
                                        getUserById={(id) => userCacheRef.current.get(id)}
                                        getSynapseById={(id) => synapseCacheRef.current.get(id)}
                                    />
                                </li>
                            ))}
                        </ul>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default SynapseActivityPanel