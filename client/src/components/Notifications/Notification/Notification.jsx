
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
        <div className="notification flex text-sm p-4 w-full
        border border-border bg-background rounded-2xl hover:bg-surface">
            <p>{summary}</p>
        </div>
    )
}

export default Notification;