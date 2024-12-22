import './NotificationsTray.css'
import Notification from '../Notification/Notification.jsx'

const NotificationsTray = ({
                                notifications,
                           }) => {

    return (
        <div className="notifications-tray">
            {notifications.length > 0 ? (
                notifications
                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                    .map((notification, index) => (
                        <Notification
                            key={index}
                            user_id={notification.user_id}
                            actor_id={notification.actor_id}
                            resource_type={notification.resource_type}
                            resource_id={notification.resource_id}
                            summary={notification.summary}
                            is_read={notification.is_read}
                            created_at={notification.created_at}

                        />
                    ))
            ) : (
                <div className="notifications-tray__default-notification">
                    <p>
                        No New Notifications
                    </p>
                </div>
            )}
        </div>
    )
}

export default NotificationsTray;