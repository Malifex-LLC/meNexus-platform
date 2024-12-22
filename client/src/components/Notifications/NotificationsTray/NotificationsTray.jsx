import './NotificationsTray.css'

const NotificationsTray = () => {
    return (
        <div className="notifications-tray">
            <ul className="notifications-tray--list">
                <li className="notifications-tray--list-item">
                    <p>Notification 1</p>
                </li>
                <li className="notifications-tray--list-item">
                    <p>Notification 2</p>
                </li>
                <li className="notifications-tray--list-item">
                    <p>Notification 3</p>
                </li>
            </ul>
        </div>
    )
}

export default NotificationsTray;