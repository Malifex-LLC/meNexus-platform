import './NotificationsTray.css'
import {useState} from 'react'
import Notification from '../Notification/Notification.jsx'

const NotificationsTray = () => {

    const [notifications, setNotifications] = useState([])


    return (
        <div className="notifications-tray">
            {notifications.length > 0 ? (
                notifications
                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                    .map((notification, index) => (
                        <Notification
                            key={index}
                            user={user_id}
                            actor_id={actor_id}
                            resource_type={resource_type}
                            resource_id={resource_id}
                            summary={summary}
                            is_read={is_read}
                            created_at={created_at}

                        />
                    ))
            ) : (
                <div>No New Notifications</div>
            )}
        </div>
    )
}

export default NotificationsTray;