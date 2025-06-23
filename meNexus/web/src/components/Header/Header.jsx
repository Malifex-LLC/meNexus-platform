import '../Search/Search.jsx'
import {useEffect, useState} from "react";
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";
import useGetNotifications from "../../api/hooks/useGetNotifications.js";
import useNotificationsWebSocket from '../../api/hooks/useNotificationsWebSocket.js'
import useSetNotificationAsRead from "../../api/hooks/useSetNotificationAsRead.js";
import { FaHome } from "react-icons/fa";
import { IoGitNetworkSharp } from "react-icons/io5";
import { FaNetworkWired } from "react-icons/fa6";
import { IoPerson } from "react-icons/io5";
import { FaEnvelope } from "react-icons/fa";
import { IoSettings } from "react-icons/io5";
import {Link, useLocation} from "react-router-dom";
import { IoNotifications } from "react-icons/io5";
import { MdOutlineSpaceDashboard } from "react-icons/md";
import { RxDashboard } from "react-icons/rx";
import { IoMdGitNetwork } from "react-icons/io";
import { RiHomeWifiLine } from "react-icons/ri";
import { RiUser6Line } from "react-icons/ri";
import { HiOutlineUser } from "react-icons/hi";
import {BsEnvelope, BsMailboxFlag} from "react-icons/bs";
import {GiSettingsKnobs} from "react-icons/gi";
import {HiOutlineBellAlert} from "react-icons/hi2";
import {LuScanSearch} from "react-icons/lu";






const Header = () => {

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getNotifications, loading, error } = useGetNotifications()
    const { setNotificationAsRead } = useSetNotificationAsRead()
    const { connectNotificationsWebSocket } = useNotificationsWebSocket();

    const [sessionUserId, setSessionUserId] = useState(null);
    const [isSessionUserIdSet, setIsSessionUserIdSet] = useState(null);
    const [showNotificationsTray, setShowNotificationsTray] = useState(false);
    const [notifications, setNotifications] = useState([]);
    const [unreadNotifications, setUnreadNotifications] = useState(false);

    const location = useLocation();
    const isActive = (path) => location.pathname.startsWith(path) ? "text-brand" : "text-foreground";


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
                        setIsSessionUserIdSet(true);
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                }
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
                newNotifications.map((notification) => {
                    if (notification.is_read === 0) {
                        setUnreadNotifications(true);
                    }
                })
            } catch (error) {
                console.error("Error fetching notifications:", error)
            }
        };
        fetchNotifications();
    }, []);

    const handleNewNotification = (notification) => {
        setNotifications((prev) => [notification, ...prev]);
    };

    const toggleNotificationsTray = () => {
        setShowNotificationsTray((prevState) => !prevState);
    }

    // TODO This cause WebSocket to connect on any page regardless of being logged in
    // TODO This causes WebSocket to constantly connect and disconnect and reconnect etc and wasn't working in prod
    // Just saw WebSocket connected while on the login page after user logout
    //console.log("useNotificationsWebSocket attempting to connect for user_id: ", sessionUserId);
    //connectNotificationsWebSocket(sessionUserId, handleNewNotification);


    return (
        <div className="header__container flex fixed top-0 left-0 w-full p-4 gap-4 justify-center
         bg-header-bg text-foreground border-b border-border z-100 shadow-xs">
            <div className={`flex-1 flex justify-center gap-8 text-3xl md:text-4xl ml-[175px]`}>
                <Link to={'/dashboard'} className={isActive('/dashboard')}>
                    <RiHomeWifiLine />
                </Link>
                <Link to={'/synapse/explore'} className={isActive('/synapse/')}>
                    <IoMdGitNetwork />
                </Link>
                <Link to={'/explore'} className={isActive('/explore')}>
                    <LuScanSearch />
                </Link>
                <Link to={'/profile'} className={isActive('/profile')}>
                    <RiUser6Line />
                </Link>
                <Link to={'/messages'} className={isActive('/messages')}>
                    <BsMailboxFlag />
                </Link>
                <Link to={'/settings'} className={isActive('/settings')}>
                    <GiSettingsKnobs />
                </Link>
                <div className="header__notifications relative  text-foreground">
                    <div
                        className={`header__notifications-tray-toggle ${
                            showNotificationsTray ? 'header__notifications-tray-toggle--expanded ' : ''
                        } ${unreadNotifications ? 'header__notifications-tray-toggle--has-notifications text-accent' : ''}`}
                        onClick={toggleNotificationsTray}
                    >
                        <div className={`text-3xl md:text-4xl cursor-pointer`}>
                            <HiOutlineBellAlert />
                        </div>
                    </div>
                    {showNotificationsTray && (
                        <div className="absolute right-0 mt-2 w-80 z-50 bg-header-bg/70 border border-border rounded-2xl shadow-lg">
                            <NotificationsTray
                                user_id={sessionUserId}
                                notifications={notifications}
                            />
                        </div>
                    )}
                </div>
            </div>
            <div className="header__search  mr-8 md:mr-0">
                <Search/>
            </div>
        </div>
    )
};

export default Header;