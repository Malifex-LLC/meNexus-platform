import './Header.css';
import '../Search/Search.jsx'
import {useEffect, useState} from "react";
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";
import useGetNotifications from "../../api/hooks/useGetNotifications.js";
import useSetNotificationAsRead from "../../api/hooks/useSetNotificationAsRead.js";

const Header = () => {

    const { getNotifications, loading, error } = useGetNotifications()
    const { setNotificationAsRead } = useSetNotificationAsRead()

    const [showNotificationsTray, setShowNotificationsTray] = useState(false);
    const [notifications, setNotifications] = useState([])

    useEffect(() => {
        const fetchNotifications = async () => {
            try {
                const newNotifications = await getNotifications();
                setNotifications(newNotifications);
                console.log("fetchNotifications retrieved:", newNotifications);
            } catch (error) {
                console.error("Error fetching notifications:", error)
            }
        };
        fetchNotifications();
    }, []);

    const toggleNotificationsTray = () => {
        setShowNotificationsTray((prevState) => !prevState);
        if (showNotificationsTray === false) {
            notifications.map((notification) => {
                setNotificationAsRead(notification.notification_id);
            })
        }
    }

    return (
        <div className="header__container">
            <header
                className='header__main-content'
                role='banner'
                aria-label='Main Header'
            >
                <h1>meNexus</h1>
            </header>
            <div className="header__search">
                <Search/>
            </div>
            <div className="header__notifications">
                <p
                    className={`header__notifications-tray-toggle ${
                        showNotificationsTray ? 'header__notifications-tray-toggle--expanded' : ''
                    } ${notifications.length > 0 ? 'header__notifications-tray-toggle--has-notifications' : ''}`}
                    onClick={toggleNotificationsTray}
                >
                    Notifications
                </p>
                {showNotificationsTray && (
                    <div className="header__notifications-tray">
                        <NotificationsTray
                        notifications={notifications}
                        />
                    </div>
                )}
            </div>
        </div>
    )
};

export default Header;