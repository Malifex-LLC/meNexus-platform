// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, { useMemo } from 'react';
import Activity from '../Activity/Activity/Activity.jsx';

// helper ────────────────
const yyyymmdd = ts => ts.slice(0, 10);                      // "2025-06-23"
const dayLabel = ts =>                                       // "Jun 23", "Apr 07"
    new Date(ts.replace(' ', 'T')).toLocaleDateString(undefined, {
        month: 'short',
        day:   '2-digit'
    });

export default function UserActivityPanel( ) {
    const activities = [
        {
            "id": "9f0f63d1-74d1-4239-938a-7ba1bcd06901",
            "actorId": "meNexus",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:a1b2c3",
            "containerType": "SYNAPSE",
            "containerId": "meNexus Builders",
            "timestamp": "2025-06-26 02:13:00"
        },
        {
            "id": "9f0f63d1-74d1-4239-938a-7ba1bcd06901",
            "actorId": "jacobwileyross",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:a1b2c3",
            "containerType": "SYNAPSE",
            "containerId": "enkiLab Synapse",
            "timestamp": "2025-06-25 10:51:00"
        },
        {
            "id": "b3e4a6d2-1c33-4d3e-8a8f-997246670902",
            "actorId": "architect",
            "verb": "COMMENTED",
            "objectType": "POST",
            "objectId": "post:a1b2c3",
            "containerType": "SYNAPSE",
            "containerId": "Axion",
            "targetType": "USER",
            "targetId": "meNexus",
            "timestamp": "2025-06-25 17:58:30",
            "meta": { "commentPreview": "Great update!" }
        },
        {
            "id": "6958d810-89d5-4df7-9cec-b7ec097e7b03",
            "actorId": "architect",
            "verb": "FOLLOWED",
            "objectType": "USER",
            "objectId": "meNexus",
            "timestamp": "2025-06-23 17:57:00"
        },
        {
            "id": "0a6de279-32f5-4f9b-bf21-47b03b83c904",
            "actorId": "architect",
            "verb": "JOINED",
            "objectType": "SYNAPSE",
            "objectId": "synapse:EnkiLab",
            "timestamp": "2025-06-23 17:55:20"
        },
        {
            "id": "52e75d50-9d6e-4e08-a05c-cfc0cb63c905",
            "actorId": "architect",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:d4e5f6",
            "containerType": "SYNAPSE",
            "containerId": "synapse:EnkiLab",
            "timestamp": "2025-06-23 17:53:55"
        },
        {
            "id": "ae320c46-17c7-41c3-8e20-eac803987506",
            "actorId": "architect",
            "verb": "LEFT",
            "objectType": "SYNAPSE",
            "objectId": "synapse:EnkiLab",
            "timestamp": "2025-06-23 17:52:10"
        },
        {
            "id": "df5b30f5-5bd7-4c70-9eb4-321ad15c3207",
            "actorId": "architect",
            "verb": "FOLLOWED",
            "objectType": "USER",
            "objectId": "heavenlyy.art",
            "timestamp": "2025-06-23 17:50:30"
        },
        {
            "id": "c5bc0192-8d03-4c8f-9b2d-4159f15d2808",
            "actorId": "architect",
            "verb": "COMMENTED",
            "objectType": "POST",
            "objectId": "post:d4e5f6",
            "containerType": "SYNAPSE",
            "containerId": "synapse:EnkiLab",
            "targetType": "USER",
            "targetId": "heavenlyy.art",
            "timestamp": "2025-06-23 17:48:55",
            "meta": { "commentPreview": "Love this perspective." }
        },
        {
            "id": "c9fbc644-bcb6-4bcb-9c21-aaef2617b509",
            "actorId": "architect",
            "verb": "UNFOLLOWED",
            "objectType": "USER",
            "objectId": "meNexus",
            "timestamp": "2025-06-23 17:47:15"
        },
        {
            "id": "d68bf461-1675-4d53-b1ef-21c409c58710",
            "actorId": "architect",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:g7h8i9",
            "containerType": "SYNAPSE",
            "containerId": "synapse:Axion",
            "timestamp": "2025-06-23 17:45:40"
        },
        {
            "id": "0b7e6b4f-5e71-4e19-acd1-4717c6561811",
            "actorId": "architect",
            "verb": "JOINED",
            "objectType": "SYNAPSE",
            "objectId": "synapse:Axion",
            "timestamp": "2025-06-23 17:44:05"
        },
        {
            "id": "b45cde38-7ee1-440e-8e52-6c166ccf1212",
            "actorId": "architect",
            "verb": "COMMENTED",
            "objectType": "POST",
            "objectId": "post:g7h8i9",
            "containerType": "SYNAPSE",
            "containerId": "synapse:Axion",
            "targetType": "USER",
            "targetId": "heavenlyy.art",
            "timestamp": "2025-06-22 17:42:25",
            "meta": { "commentPreview": "Appreciate the insight!" }
        },
        {
            "id": "b9fd685e-017e-4b05-bea4-45fdf93bb713",
            "actorId": "architect",
            "verb": "FOLLOWED",
            "objectType": "USER",
            "objectId": "meNexus",
            "timestamp": "2025-06-22 17:40:40"
        },
        {
            "id": "f66916bb-0e6a-4230-8ee7-6c2722f5b814",
            "actorId": "architect",
            "verb": "LEFT",
            "objectType": "SYNAPSE",
            "objectId": "synapse:Axion",
            "timestamp": "2025-06-22 17:38:55"
        },
        {
            "id": "c711fd41-cba1-4df8-9af3-37cee6e21c15",
            "actorId": "architect",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:j1k2l3",
            "containerType": "SYNAPSE",
            "containerId": "synapse:EnkiLab",
            "timestamp": "2025-06-22 17:37:20"
        },
        {
            "id": "fa67d30e-8c0b-40a5-9e3d-5308eb3eba16",
            "actorId": "architect",
            "verb": "COMMENTED",
            "objectType": "POST",
            "objectId": "post:j1k2l3",
            "containerType": "SYNAPSE",
            "containerId": "synapse:EnkiLab",
            "targetType": "USER",
            "targetId": "meNexus",
            "timestamp": "2025-06-22 17:35:50",
            "meta": { "commentPreview": "Thanks for sharing!" }
        },
        {
            "id": "a4f854c6-b2c5-4adb-8907-5f2fbfe7e217",
            "actorId": "architect",
            "verb": "FOLLOWED",
            "objectType": "USER",
            "objectId": "heavenlyy.art",
            "timestamp": "2025-06-21 17:34:05"
        },
        {
            "id": "d739c5e8-7799-4d9f-b6d2-e8a1fef7b318",
            "actorId": "architect",
            "verb": "UNFOLLOWED",
            "objectType": "USER",
            "objectId": "heavenlyy.art",
            "timestamp": "2025-06-21 17:32:25"
        },
        {
            "id": "3879244f-5491-42dd-a43b-6f10b6b6b419",
            "actorId": "architect",
            "verb": "JOINED",
            "objectType": "SYNAPSE",
            "objectId": "synapse:Axion",
            "timestamp": "2025-06-21 17:30:40"
        },
        {
            "id": "ebacbe83-9f0d-497e-8d8c-c7cf271efc20",
            "actorId": "architect",
            "verb": "LEFT",
            "objectType": "SYNAPSE",
            "objectId": "synapse:EnkiLab",
            "timestamp": "2025-06-21 17:29:05"
        },
        {
            "id": "f0cb633e-d4e1-4c9f-ae00-97958a9a1121",
            "actorId": "architect",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:m4n5o6",
            "containerType": "SYNAPSE",
            "containerId": "synapse:Axion",
            "timestamp": "2025-06-21 17:27:30"
        },
        {
            "id": "bbdcb94e-4e97-4d7f-9ccb-73fab71bc122",
            "actorId": "architect",
            "verb": "COMMENTED",
            "objectType": "POST",
            "objectId": "post:m4n5o6",
            "containerType": "SYNAPSE",
            "containerId": "synapse:Axion",
            "targetType": "USER",
            "targetId": "heavenlyy.art",
            "timestamp": "2025-06-21 17:25:45",
            "meta": { "commentPreview": "Interesting point!" }
        },
        {
            "id": "574dfa0c-d4af-45da-8b79-3d7285ea9323",
            "actorId": "architect",
            "verb": "FOLLOWED",
            "objectType": "USER",
            "objectId": "meNexus",
            "timestamp": "2025-06-20 17:24:00"
        },
        {
            "id": "f65cc3d3-12bb-4284-9b7f-3c478c5d7424",
            "actorId": "architect",
            "verb": "JOINED",
            "objectType": "SYNAPSE",
            "objectId": "synapse:EnkiLab",
            "timestamp": "2025-06-20 17:22:25"
        },
        {
            "id": "89a14851-623e-4fd3-a67e-6baa6ebe5c25",
            "actorId": "architect",
            "verb": "POSTED",
            "objectType": "POST",
            "objectId": "post:p7q8r9",
            "containerType": "SYNAPSE",
            "containerId": "synapse:EnkiLab",
            "timestamp": "2025-06-20 17:20:50"
        }
    ]

    /* 1 ▸ group by day */
    const groups = useMemo(() => {
        return activities.reduce((acc, a) => {
            const key = yyyymmdd(a.timestamp);
            (acc[key] ||= []).push(a);
            return acc;
        }, {});
    }, [activities]);

    /* 2 ▸ render */
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
                                {dayLabel(items[0].timestamp)}
                            </span>
                        </div>

                        {/* items for that day */}
                        <ul className={'px-8 py-2'}>
                            {items.map((activity, i) => (
                                <li key={i} className="relative pl-8 py-16 ">
                                    {/* dot */}
                                    <span className="absolute left-3 top-6 w-3 h-3 rounded-full bg-brand" />
                                    <Activity activity={activity} />
                                </li>
                            ))}
                        </ul>
                    </li>
                ))}
            </ul>
        </div>
    );
}
