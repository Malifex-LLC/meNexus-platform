// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import useGetUser from "../../../api/hooks/useGetUser.js";
import { Link } from "react-router-dom";
import useSetNotificationAsRead from "../../../api/hooks/useSetNotificationAsRead.js";

const Notification = ({
                          notification_id,
                          public_key,
                          actor_public_key,
                          resource_type,
                          resource_id,
                          summary,
                          is_read,
                          created_at,
                      }) => {
    const { getUser } = useGetUser();
    const { setNotificationAsRead } = useSetNotificationAsRead();
    const [actorUser, setActorUser] = useState(null);

    useEffect(() => {
        if (resource_type === "FOLLOW" && actor_public_key) {
            const fetchActor = async () => {
                const user = await getUser(actor_public_key);
                setActorUser(user);
            };
            fetchActor();
        }
    }, [actor_public_key, resource_type]);

    const renderNotification = () => {
        switch (resource_type) {
            case "FOLLOW":
                return actorUser ? (
                        <Link to={`/profile/${actorUser.handle}`}
                              onClick={() => setNotificationAsRead(notification_id)}
                        >
                            <p>{summary}</p>
                        </Link>
                ) : (
                    <p>Loading...</p>
                );

            case "POST":
                return (
                    <Link to={`/post/${resource_id}`}
                          onClick={() => setNotificationAsRead(notification_id)}
                    >
                        <p>{summary}</p>
                    </Link>
                );

            default:
                return <p>{summary}</p>;
        }
    };

    return (
        <div className="notification flex text-sm p-4 w-full border border-border bg-background rounded-2xl hover:bg-surface">
            {renderNotification()}
        </div>
    );
};

export default Notification;
