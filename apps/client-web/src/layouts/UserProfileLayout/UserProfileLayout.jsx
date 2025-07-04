// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import {useEffect, useState} from "react";
import {useNavigate, useParams} from "react-router-dom"
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUserByHandle from "../../api/hooks/useGetUserByHandle.js";
import IdentityPanel from "../../components/IdentityPanel/IdentityPanel.jsx";
import UserOverviewPanel from "../../components/UserOverviewPanel/UserOverviewPanel.jsx";
import UserShowcasePanel from "../../components/UserShowcasePanel/UserShowcasePanel.jsx";
import UserActivityPanel from "../../components/UserActivityPanel/UserActivityPanel.jsx";
import UserJoinedSynapsesPanel from "../../components/UserJoinedSynapsesPanel/UserJoinedSynapsesPanel.jsx";

const UserProfileLayout = ({ children }) => {
    const navigate = useNavigate(); // React Router navigate
    const { handle } = useParams();

    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const [currentHandle, setCurrentHandle ] = useState(null)
    const { getUserByHandle } = useGetUserByHandle();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const [activeTab, setActiveTab] = useState("overview");
    const [isProfileOwner, setIsProfileOwner] = useState(false)

    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                console.log("Fetching current user session...");
                const response = await getSessionUser();
                setSessionUser(response.data)
                if (!handle) {
                    navigate(`/profile/${sessionUser.handle}`)
                }
                if (response.data.handle === handle) {
                    setIsProfileOwner(true)
                }
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
                const response = await getUserByHandle(handle);
                setUser(response);
            } catch (error) {
                console.error("Error fetching current user:", error);
            }
        }
        fetchUser();
    }, [sessionUser])

    if (!user || !user.publicKey) {
        return <div className={'h-screen bg-background'}></div>;
    }

    return (
        <div className={''}>
            {/* Remove the bg-background above to see the magic below */}
            {/*<div className={`absolute top-0 left-o -z-1 pt-17 h-screen w-screen bg-gradient-to-b from-background via-primary to-background backdrop-blur-lg `}/>*/}

            <div className='h-screen flex flex-col'>
                {/* Header */}
                <div className={`sticky top-0 z-50 h-17 shrink-0 border-b border-border`}>
                    <Header
                        user={user}
                    />
                </div>
                {/* IdentityPanel */}
                <div className="flex-1 w-full flex flex-col lg:grid lg:grid-cols-12 overflow-hidden">
                    <div className={'flex-col flex-1 min-h-0 overflow-y-auto p-4  w-full lg:col-span-4'}>
                        <IdentityPanel
                            user={user}
                            isProfileOwner={isProfileOwner}
                        />
                    </div>
                    {/* Main Content Grid */}
                    <div className='flex flex-col h-full min-h-0 col-span-8'>
                        {/* Tab Switcher */}
                        <div className="flex flex-row bg-background  justify-around border border-border p-2 m-4 text-xl text-foreground shadows-2xl rounded-xl col-span-8">
                            <button
                                onClick={() => setActiveTab("overview")}
                                className={`${activeTab === "overview" ? "text-brand font-bold" : "hover:text-brand"}`}
                            >
                                Overview
                            </button>
                            <button
                                onClick={() => setActiveTab("showcase")}
                                className={`${activeTab === "showcase" ? "text-brand font-bold" : "hover:text-brand"}`}
                            >
                                Showcase
                            </button>
                            <button
                                onClick={() => setActiveTab("synapses")}
                                className={`${activeTab === "synapses" ? "text-brand font-bold" : "hover:text-brand"}`}
                            >
                                Synapses
                            </button>
                            <button
                                onClick={() => setActiveTab("activity")}
                                className={`${activeTab === "activity" ? "text-brand font-bold" : "hover:text-brand"}`}
                            >
                                Activity
                            </button>
                        </div>
                        <div className={'flex flex-col h-full min-h-0 '}>
                            {activeTab === "overview" ? (
                                <UserOverviewPanel
                                    user={user}
                                />
                            ) : activeTab === "showcase" ? (
                                <UserShowcasePanel/>
                            ) : activeTab === "activity" ? (
                                <UserActivityPanel/>
                            ) : activeTab === "synapses" ? (
                                <UserJoinedSynapsesPanel user={user} />
                            ) : (
                                <div>Selected Tab Not Valid</div>
                            )}

                        </div>

                        {children}
                    </div>
                </div>

            </div>
        </div>
);
};

export default UserProfileLayout;