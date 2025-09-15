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
import {useNavigate, useParams} from "react-router-dom";
import {TbLogout2} from "react-icons/tb";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";

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
    const { getSynapseMetadata } = useGetSynapseMetadata();

    const [localSynapseMetadata, setLocalSynapseMetadata] = useState(null);
    const navigate = useNavigate(); // React Router navigate
    const { synapsePublicKey } = useParams(); // Extract synapsePublicKey from the URL (if available)


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

    useEffect(() => {
        const fetchSynapseMetadata = async () => {
            const localSynapseData = await getSynapseMetadata();
            setLocalSynapseMetadata(localSynapseData)
        };
        fetchSynapseMetadata();
    },[synapsePublicKey]);


    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className='flex flex-col h-[100dvh] sm:items-center xl:items-start  bg-background '>
            <div className='sticky top-0 z-50 border-b border-border w-full'>
                <Header
                    user={user}
                    localSynapseMetadata={localSynapseMetadata}
                />
                {/* Mobile Nav */}
                <div className='flex  w-full xl:hidden pt-17 py-2 justify-evenly border-b bg-background border-border text-foreground'>
                    <button
                        onClick={() => setActivePanel(0)}
                        className={`${activePanel === 0 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Profile
                    </button>
                    <button
                        onClick={() => setActivePanel(1)}
                        className={`${activePanel === 1 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Account
                    </button>
                    <button
                        onClick={() => setActivePanel(2)}
                        className={`${activePanel === 2 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Display
                    </button>
                    <button
                        /* TODO implement real logout function/destroy session instead of navigating to login */
                        className={`flex items-center text-xl text-foreground hover:text-brand/60 hover:cursor-pointer`}
                        onClick={() => navigate('/login')}
                    >
                    <TbLogout2 />
                    </button>
                </div>
            </div>

            {/* Content Area */}
            <div className={`flex h-full w-full xl:pt-17 overflow-y-auto`}>
                <div className="hidden xl:flex   text-foreground">
                    {/* Render Settings Menu */}
                    {children[0]}
                </div>
                {/** MOBILE SWIPEABLE VIEWPORT */}
                <div className="xl:hidden w-full">
                    <div className="flex flex-1 w-full items-center  overflow-y-auto" {...swipeHandlers}>
                        <main className="settings-layout__main-content flex flex-1 w-full ">
                            <div className={`${activePanel === 0 ? 'flex' : 'hidden'} w-full flex-col`}>
                                <ProfileSettings />
                            </div>
                            <div className={`${activePanel === 1 ? 'flex' : 'hidden'} w-full`}>
                                <AccountSettings />
                            </div>
                            <div className={`${activePanel === 2 ? 'flex' : 'hidden'} w-full flex-col`}>
                                <DisplaySettings />
                            </div>
                        </main>
                    </div>
                </div>

                <div className="hidden xl:flex flex-1 ">
                    {children[1]}
                </div>
            </div>

        </div>
    );
}

export default SettingsLayout;