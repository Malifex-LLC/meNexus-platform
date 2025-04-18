import '../Search/Search.jsx'
import {useEffect, useState} from "react";
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";
import useGetNotifications from "../../api/hooks/useGetNotifications.js";
import useNotificationsWebSocket from '../../api/hooks/useNotificationsWebSocket.js'
import useSetNotificationAsRead from "../../api/hooks/useSetNotificationAsRead.js";
import { FaHome } from "react-icons/fa";
import { IoPerson } from "react-icons/io5";
import { FaEnvelope } from "react-icons/fa";
import { IoSettings } from "react-icons/io5";
import {Link, useLocation} from "react-router-dom";
import { IoNotifications } from "react-icons/io5";

const Header = () => {

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getNotifications, loading, error } = useGetNotifications()
    const { setNotificationAsRead } = useSetNotificationAsRead()
    const { connectNotificationsWebSocket } = useNotificationsWebSocket();

    const [sessionUserId, setSessionUserId] = useState(null);
    const [isSessionUserIdSet, setIsSessionUserIdSet] = useState(null);
    const [showNotificationsTray, setShowNotificationsTray] = useState(false);
    const [notifications, setNotifications] = useState([])

    const location = useLocation();
    const isActive = (path) => location.pathname === path ? "text-brand" : "text-foreground";


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
    // TODO This causes WebSocket to constantly connect and disconnect and reconnect etc and wasn't working in prod
    // Just saw WebSocket connected while on the login page after user logout
    //console.log("useNotificationsWebSocket attempting to connect for user_id: ", sessionUserId);
    //connectNotificationsWebSocket(sessionUserId, handleNewNotification);


    return (
        <div className="header__container flex fixed top-0 left-0 w-full p-4 gap-4 justify-center
         bg-header-bg text-foreground z-100">
            <div className={`flex-1 flex justify-center gap-8 text-4xl ml-[175px]`}>
                <Link to={'/home'} className={isActive('/home')}>
                    <FaHome />
                </Link>
                <Link to={'/profile'} className={isActive('/profile')}>
                    <IoPerson />
                </Link>
                <Link to={'/messages'} className={isActive('/messages')}>
                    <FaEnvelope />
                </Link>
                <Link to={'/settings'} className={isActive('/settings')}>
                    <IoSettings />
                </Link>
                <div className="header__notifications relative  text-foreground">
                    <div
                        className={`header__notifications-tray-toggle ${
                            showNotificationsTray ? 'header__notifications-tray-toggle--expanded ' : ''
                        } ${notifications.length > 0 ? 'header__notifications-tray-toggle--has-notifications text-red-500' : ''}`}
                        onClick={toggleNotificationsTray}
                    >
                        <div className={`text-4xl`}>
                            <IoNotifications />
                        </div>

                    </div>
                    {showNotificationsTray && (
                        <div className={`header__notifications-tray absolute bg-black rounded-2xl  top-13  z-10 w-md`}>
                            <NotificationsTray
                                user_id={sessionUserId}
                                existingNotifications={notifications}
                            />
                        </div>
                    )}
                </div>
            </div>

            <div className="header__search ml-auto  ">
                <Search/>
            </div>
        </div>
    )
};

export default Header;