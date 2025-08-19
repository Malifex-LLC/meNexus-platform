// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useActivityWebSocket from "../../api/hooks/useActivityWebSocket.js";
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import {useParams} from "react-router-dom";

const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

const  SynapseActivityPanel = ({isLocalSynapse, synapseMetadata, publicKey}) => {
    const { synapsePublicKey } = useParams();
    const [activities, setActivities] = useState(null);
    const [bufferedActivities, setBufferedActivities] = useState([]);
    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();

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
        <div className="flex flex-1  overflow-y-auto bg-background rounded-xl  text-foreground">

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
                            <span className="px-2 py-0.5 bg-surface font-medium text-3xl text-neutral shadow-2xl rounded-xl"
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
                                    <Activity activity={activity} mode={'SYNAPSE'} />
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
