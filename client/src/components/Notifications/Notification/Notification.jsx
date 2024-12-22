import './Notification.css'

const Notification = ({
                          notification_id,
                          user_id,
                          actor_id,
                          resource_type,
                          resource_id,
                          summary,
                          is_read,
                          created_at,
                      }) => {

    return (
        <div className="notification">
            <p>Test Notification</p>
        </div>
    )
}

export default Notification;