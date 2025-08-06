// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useState} from 'react';
import Activity from '../Activity/Activity/Activity.jsx';
import useGetAllActivities from '../../api/hooks/useGetAllActivities.js'
import useFetchRemoteSynapseAllActivities from "../../api/hooks/useFetchRemoteSynapseAllActivities.js";
import {useParams} from "react-router-dom";

const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

const  SynapseActivityPanel = ({isLocalSynapse}) => {
    const { synapsePublicKey } = useParams();
    const [activities, setActivities] = useState([]);
    const { getAllActivities } = useGetAllActivities();
    const { fetchRemoteSynapseAllActivities } = useFetchRemoteSynapseAllActivities();

    useEffect(() => {
        const getActivites = async () => {
            if (isLocalSynapse) {
                const activitiesData = await getAllActivities();
                setActivities(activitiesData)
            } else {
                const activitiesData = await fetchRemoteSynapseAllActivities(synapsePublicKey);
                setActivities(activitiesData)
            }

        }
        getActivites();
    }, [synapsePublicKey])


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
        <div className="flex-1 min-h-0 overflow-y-auto bg-background rounded-xl m-4 text-foreground">

            {/* master timeline line */}
            <ul className="p-4">
                {Object.entries(groups).map(([day, items]) => (
                    <li key={day} className="">
                        {/* day badge */}
                        <div className="">
                            <span className="px-2 py-0.5 bg-surface font-medium text-3xl text-neutral shadow-2xl rounded-xl"
                            >
                                {dayLabel(items[0].published)}
                            </span>
                        </div>

                        {/* items for that day */}
                        <ul className={'px-8 py-2'}>
                            {items.map((activity, i) => (
                                <li key={i} className="relative pl-8 py-16 ">
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
