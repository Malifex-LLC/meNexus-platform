// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useActivityWebSocket from "../../api/hooks/useActivityWebSocket.js";
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";

const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

const  GlobalActivityPanel = ({user, localSynapseMetadata}) => {
    const [localSynapse, setLocalSynapse] = useState({})
    const [activities, setActivities] = useState([]);
    const [bufferedActivities, setBufferedActivities] = useState([]);
    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();
    const publicKey = user.publicKey;

    // TODO This method only receives new activities for the local Synapse in realtime, need to implement a pub/sub for all Synapses
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

    useEffect(() => {
        const aggregateActivity = async () => {
            console.log('Aggregate Activity called');
            if (!user || !user.synapses) return;

            const allPostPromises = user.synapses.map(async (synapse) => {
                if (synapse === localSynapseMetadata.identity.publicKey) {
                    return await getAllActivities(); // returns array
                } else {
                    return await fetchRemoteSynapseAllActivities(synapse); // returns array
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
        <div className="flex flex-col min-h-0 h-full  bg-surface/70   text-foreground border border-border rounded-xl">
            {/* master timeline line */}
            <div className="flex justify-around p-4 gap-4 bg-surface border-b border-border rounded-t-xl text-2xl text-foreground shadows-2xl ">
                <button className={`text-brand font-bold `}>
                    Activity
                </button>
            </div>
            <ul className="p-4 overflow-y-auto">
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
                        <ul className={'px-8 py-2'}>
                            {items.map((activity, i) => (
                                <li key={i} className="relative pl-8 py-8 ">
                                    {/* dot */}
                                    <span className="absolute left-3 top-6 w-3 h-3 rounded-full bg-brand" />
                                    <Activity activity={activity} mode={'GLOBAL'} />
                                </li>
                            ))}
                        </ul>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default GlobalActivityPanel