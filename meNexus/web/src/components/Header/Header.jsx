import './Header.css';
import '../Search/Search.jsx'
import {useEffect, useState} from "react";
import useGetSessionUser from '../../hooks/api/useGetSessionUser.js'
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";
import useGetNotifications from "../../hooks/api/useGetNotifications.js";
import useNotificationsWebSocket from '../../hooks/api/useNotificationsWebSocket.js'
import useSetNotificationAsRead from "../../hooks/api/useSetNotificationAsRead.js";

const Header = () => {

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getNotifications, loading, error } = useGetNotifications()
    const { setNotificationAsRead } = useSetNotificationAsRead()
    const { connectNotificationsWebSocket } = useNotificationsWebSocket();

    const [sessionUserId, setSessionUserId] = useState(null);
    const [isSessionUserIdSet, setIsSessionUserIdSet] = useState(null);
    const [showNotificationsTray, setShowNotificationsTray] = useState(false);
    const [notifications, setNotifications] = useState([])

    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!sessionUserId && !isSessionUserIdSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();
                    console.log(response);

                    if (response.status === 200 && response.data.user_id) {
                        console.log("Session user handle:", response.data.handle);
                        setSessionUserId(response.data.user_id);
                    } else {
                        console.error("Invalid session");
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                }
            } else if (sessionUserId) {
                setSessionUserId(sessionUserId);
            }
        };

        fetchSessionUser();
    }, [sessionUserId, isSessionUserIdSet]);

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

    const handleNewNotification = (notification) => {
        setNotifications((prev) => [notification, ...prev]);
    };

    // TODO This seems atrocious but its working idk
    const toggleNotificationsTray = () => {
        setShowNotificationsTray((prevState) => !prevState);
        if (showNotificationsTray === true) {
            notifications.map((notification) => {
                setNotificationAsRead(notification.notification_id);
            })
            notifications.length = 0;
        }
        if (!showNotificationsTray) {
            getNotifications().then((fetchedNotifications) => {
                setNotifications(fetchedNotifications);
                notifications.map((notification) => {
                    setNotificationAsRead(notification.notification_id);
                })
            });
        }
    }

    // TODO This cause WebSocket to connect on any page regardless of being logged in
    // Just saw WebSocket connected while on the login page after user logout
    console.log("useNotificationsWebSocket attempting to connect for user_id: ", sessionUserId);
    connectNotificationsWebSocket(sessionUserId, handleNewNotification);

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
                            user_id={sessionUserId}
                            existingNotifications={notifications}
                        />
                    </div>
                )}
            </div>
        </div>
    )
};

export default Header;