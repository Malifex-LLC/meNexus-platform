// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import '../Search/Search.jsx'
import {useEffect, useState} from "react";
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";
import useGetNotifications from "../../api/hooks/useGetNotifications.js";
import useNotificationsWebSocket from '../../api/hooks/useNotificationsWebSocket.js'
import useSetNotificationAsRead from "../../api/hooks/useSetNotificationAsRead.js";
import { NavLink, useLocation } from "react-router-dom";
import { IoMdGitNetwork } from "react-icons/io";
import { RiHomeWifiLine } from "react-icons/ri";
import { RiUser6Line } from "react-icons/ri";
import { BsMailboxFlag } from "react-icons/bs";
import { GiSettingsKnobs } from "react-icons/gi";
import { HiOutlineBellAlert } from "react-icons/hi2";
import { LuScanSearch } from "react-icons/lu";

const Header = ({user, localSynapseMetadata}) => {

    const { getNotifications, loading, error } = useGetNotifications()
    const { setNotificationAsRead } = useSetNotificationAsRead()
    const { connectNotificationsWebSocket } = useNotificationsWebSocket();

    const [showNotificationsTray, setShowNotificationsTray] = useState(false);
    const [notifications, setNotifications] = useState([]);
    const [unreadNotifications, setUnreadNotifications] = useState(false);

    const location = useLocation();
    const isActive = (path) => location.pathname.startsWith(path) ? "text-brand" : "text-foreground active:scale-90";



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

    if(!localSynapseMetadata){
        return (
            <div className={'bg-header-bg text-foreground'}></div>
        )
    }

    return (
        <div className="header__container flex fixed top-0 left-0 w-full p-4 gap-4 justify-center
         bg-header-bg text-foreground border-b border-border z-100 shadow-xs h-16">
            <div className={`flex-1 flex justify-center gap-8 text-3xl md:text-4xl `}>
                <NavLink to={'/dashboard'} className={isActive('/dashboard')}>
                    <RiHomeWifiLine />
                </NavLink>
                <NavLink to={`/synapse/${localSynapseMetadata.identity.publicKey}`} className={isActive('/synapse/')}>
                    <IoMdGitNetwork />
                </NavLink>
                <NavLink to={'/explore'} className={isActive('/explore')}>
                    <LuScanSearch />
                </NavLink>
                <NavLink to={`/profile/${user.handle}`} className={isActive('/profile')}>
                    <RiUser6Line />
                </NavLink>
                {/*<Link to={'/messages'} className={isActive('/messages')}>*/}
                {/*    <BsMailboxFlag />*/}
                {/*</Link>*/}
                <NavLink to={'/settings'} className={isActive('/settings')}>
                    <GiSettingsKnobs />
                </NavLink>
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
                        <div className="absolute right-0 mt-2 w-80 z-50 bg-header-bg/70 backdrop-blur-xs border border-border rounded-2xl shadow-lg">
                            <NotificationsTray
                                user_id={user.publicKey}
                                notifications={notifications}
                            />
                        </div>
                    )}
                </div>
            </div>
            {/*<div className="header__search  mr-8 md:mr-0">*/}
            {/*    <Search/>*/}
            {/*</div>*/}
        </div>
    )
};

export default Header;