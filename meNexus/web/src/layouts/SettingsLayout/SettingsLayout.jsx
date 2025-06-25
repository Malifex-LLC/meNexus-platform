// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";
import {useEffect, useState} from "react";
import {useSwipeable} from "react-swipeable";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import ProfileSettings from "../../components/Settings/ProfileSettings/ProfileSettings.jsx";
import AccountSettings from "../../components/Settings/AccountSettings/AccountSettings.jsx";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import DisplaySettings from "../../components/Settings/DisplaySettings/DisplaySettings.jsx";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate} from "react-router-dom";

const SettingsLayout = ({ children }) => {
    const [activePanel, setActivePanel] = useState(0); // 0: Profile, 1: Account, 2: Display
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActivePanel((prev) => Math.min(prev + 1, 2)),
        onSwipedRight: () => setActivePanel((prev) => Math.max(prev - 1, 0)),
    });

    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const navigate = useNavigate(); // React Router navigate


    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                console.log("Fetching current user session...");
                const response = await getSessionUser();
                setSessionUser(response.data)
            } catch (error) {
                console.error("Error fetching current session user:", error);
                navigate('/login');
            }
        }
        fetchSessionUser();
    }, [])

    useEffect(() => {
        const fetchUser = async () => {
            try {
                const response = await getUser(sessionUser.publicKey);
                setUser(response);
            } catch (error) {
                console.error("Error fetching current user:", error);
            }
        }
        fetchUser();
    }, [sessionUser])


    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className='settings-layout h-screen  flex flex-col sm:items-center xl:items-start  bg-background '>
            <div className='sticky top-0 z-50 border-b border-border w-full'>
                <Header
                    user={user}
                />
                {/* Mobile Nav */}
                <div className='flex  w-full lg:hidden pt-17 py-2 justify-evenly border-b bg-background border-border text-foreground'>
                    <button
                        onClick={() => setActivePanel(0)}
                        className={`${activePanel === 0 ? 'text-[#FF6B6B] font-semibold' : 'text-foreground'}`}
                    >
                        Profile
                    </button>
                    <button
                        onClick={() => setActivePanel(1)}
                        className={`${activePanel === 1 ? 'text-[#FF6B6B] font-semibold' : 'text-foreground'}`}
                    >
                        Account
                    </button>
                    <button
                        onClick={() => setActivePanel(2)}
                        className={`${activePanel === 2 ? 'text-[#FF6B6B] font-semibold' : 'text-foreground'}`}
                    >
                        Display
                    </button>
                </div>
            </div>

            {/* Content Area */}
            <div className={`flex h-full pt-17 px-8 overflow-y-auto`}>
                <div className="settings-layout__menu hidden lg:flex   text-foreground">
                    {/* Render Settings Menu */}
                    {children[0]}
                </div>
                {/** MOBILE SWIPEABLE VIEWPORT */}
                {window.innerWidth < 1050 && (
                    <div className="lg:hidden w-full">
                        <div className="flex flex-1 w-full items-center  overflow-y-auto" {...swipeHandlers}>
                            <main className="settings-layout__main-content flex flex-1 ">
                                <div className={`${activePanel === 0 ? 'flex w-full items-center' : 'hidden'} min-h-screen flex-col  pt-17 `}>
                                    <ProfileSettings />
                                </div>
                                <div className={`${activePanel === 1 ? 'flex' : 'hidden'} min-h-screen pt-17 `}>
                                    <AccountSettings />
                                </div>
                                <div className={`${activePanel === 2 ? 'flex' : 'hidden'} min-h-screen flex-col pt-17 `}>
                                    <DisplaySettings />
                                </div>
                            </main>
                        </div>
                    </div>
                )}

                <div className="hidden lg:flex flex-1 ">
                    {children[1]}
                </div>
            </div>

        </div>
    );
}

export default SettingsLayout;