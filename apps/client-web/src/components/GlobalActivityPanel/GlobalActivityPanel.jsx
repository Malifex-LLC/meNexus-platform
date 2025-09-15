import React, {useEffect, useMemo, useRef, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useActivityWebSocket from "../../api/hooks/useActivityWebSocket.js";
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetUser from "../../api/hooks/useGetUser.js";

const yyyymmdd = ts => ts.slice(0, 10);
const dayLabel = ts =>
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, { month: 'short', day: '2-digit' });

const GlobalActivityPanel = ({ user, localSynapseMetadata }) => {
    const [activities, setActivities] = useState([]);
    const [bufferedActivities, setBufferedActivities] = useState([]);

    const synapseCacheRef = useRef(new Map()); // key: publicKey, val: metadata
    const userCacheRef    = useRef(new Map()); // key: publicKey, val: user

    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { getUser } = useGetUser();

    // TODO This method only receives new activities for the local Synapse in realtime, need to implement a pub/sub for all Synapses
    const publicKey = user.publicKey;
    useActivityWebSocket({
        wsUrl: localSynapseMetadata.identity.webSocketUrl,
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
        const aggregateActivity = async () => {
            if (!user?.synapses) return;

            const tasks = user.synapses.map(async (synapsePublicKey) => {
                if (synapsePublicKey === localSynapseMetadata.identity.publicKey) return getAllActivities();
                return fetchRemoteSynapseAllActivities(synapsePublicKey);
            });

            const results = await Promise.all(tasks);
            const combined = results.flat().sort((a, b) => new Date(b.published) - new Date(a.published));
            setActivities(combined);
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

    // Group by day
    const groups = useMemo(() => {
        return activities.reduce((acc, a) => {
            const key = yyyymmdd(a.published);
            (acc[key] ||= []).push(a);
            return acc;
        }, {});
    }, [activities]);

    if (!activities) return <div>Loading activities...</div>;

    return (
        <div className="flex flex-col min-h-0 h-full bg-surface/70 text-foreground border border-border xl:rounded-xl">
            <div className="flex justify-around p-4 gap-4 bg-surface border-b border-border rounded-t-xl text-2xl text-foreground shadows-2xl">
                <button className="text-brand font-bold">Activity</button>
            </div>

            <ul className="p-4 overflow-y-auto">
                {Object.entries(groups).map(([day, items]) => (
                    <li key={day}>
                        <div>
              <span className="px-2 py-0.5 bg-background font-medium text-3xl text-neutral shadow-2xl rounded-xl">
                {dayLabel(items[0].published)}
              </span>
                        </div>

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
};

export default GlobalActivityPanel;
