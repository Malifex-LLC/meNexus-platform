// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useEffect, useState} from "react";
import Notification from '../Notification/Notification.jsx'

const NotificationsTray = ({
                                user_id,
                                notifications,
                           }) => {

    return (
        <div className="notifications-tray flex flex-col  m-4   text-brand rounded-xl">
            {notifications.length > 0 ? (
                notifications
                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                    .map((notification, index) => (
                        <div key={index} className={`my-2`}>
                            <Notification
                                notification_id={notification.notification_id}
                                public_key={notification.public_key}
                                actor_public_key={notification.actor_public_key}
                                resource_type={notification.resource_type}
                                resource_id={notification.resource_id}
                                summary={notification.summary}
                                is_read={notification.is_read}
                                created_at={notification.created_at}
                            />
                        </div>
                    ))
            ) : (
                <div className="notifications-tray__default-notification flex w-full text-md">
                    <Notification
                        summary={`No New Notifications`}
                    />
                </div>
            )}
        </div>
    )
}

export default NotificationsTray;