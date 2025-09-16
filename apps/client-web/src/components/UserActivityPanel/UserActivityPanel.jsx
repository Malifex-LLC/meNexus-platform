// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useRef, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import useActivityWebSocket from "../../api/hooks/useActivityWebSocket.js";
import useGetUserActivities from "../../api/hooks/useGetUserActivities.js";
import useFetchRemoteSynapseUserActivities from "../../api/hooks/useFetchRemoteSynapseUserActivities.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetUser from "../../api/hooks/useGetUser.js";

const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

export default function UserActivityPanel({user, localSynapseMetadata}) {
    const [localSynapse, setLocalSynapse] = useState({})
    const [activities, setActivities] = useState([]);
    const [bufferedActivities, setBufferedActivities] = useState([]);

    const synapseCacheRef = useRef(new Map()); // key: publicKey, val: metadata
    const userCacheRef    = useRef(new Map()); // key: publicKey, val: user

    const { getUserActivities } = useGetUserActivities();
    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseUserActivities } = useFetchRemoteSynapseUserActivities();
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { getUser } = useGetUser();

    const publicKey = user.publicKey;

    // Aggregate activities from all followed synapses
    useEffect(() => {
        const aggregateActivity = async () => {
            console.log('Aggregate Activity called');
            if (!user || !user.synapses) return;

            const allPostPromises = user.synapses.map(async (synapse) => {
                if (synapse === localSynapseMetadata.identity.publicKey) {
                    return await getUserActivities(user.publicKey); // returns array
                } else {
                    return await fetchRemoteSynapseUserActivities(synapse, user.publicKey); // returns array
                }
            });

            const results = await Promise.all(allPostPromises);
            const combinedActivities = results
                .flat() // flatten array of arrays
                .sort((a, b) => new Date(b.published) - new Date(a.published));

            setActivities(combinedActivities);
            console.log('Aggregated activities:', combinedActivities);
        };

        aggregateActivity();
    }, [user]);

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


    /* group by day */
    const groups = useMemo(() => {
        return activities.reduce((acc, a) => {
            const key = yyyymmdd(a.published);
            (acc[key] ||= []).push(a);
            return acc;
        }, {});
    }, [activities]);


    if (!activities) {
        return <div>Loading activities...</div>
    }
    return (
        <div className="flex-1 w-full min-h-0 overflow-y-auto bg-surface/70 border border-border rounded-xl text-foreground">

            {/* master timeline line */}
            <ul className="p-4">
                {Object.entries(groups).map(([day, items]) => (
                    <li key={day} className="">
                        {/* day badge */}
                        <div className="">
                            <span className="px-2 py-0.5 bg-background font-medium text-3xl text-neutral shadow-2xl rounded-xl"
                            >
                                {dayLabel(items[0].published)}
                            </span>
                        </div>

                        {/* items for that day */}
                        <ul className="px-8 py-2">
                            {items.map((activity, i) => (
                                <li key={`${activity.object_id || activity.published}-${i}`} className="relative pl-8 py-8">
                                    <span className="absolute left-3 top-6 w-3 h-3 rounded-full bg-brand" />
                                    <Activity
                                        activity={activity}
                                        mode="GLOBAL"
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
