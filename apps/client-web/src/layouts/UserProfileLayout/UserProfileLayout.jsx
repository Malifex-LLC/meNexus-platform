// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useEffect, useState} from "react";
import {useNavigate, useOutletContext, useParams} from "react-router-dom"
import IdentityPanel from "../../components/IdentityPanel/IdentityPanel.jsx";
import UserOverviewPanel from "../../components/UserOverviewPanel/UserOverviewPanel.jsx";
import UserShowcasePanel from "../../components/UserShowcasePanel/UserShowcasePanel.jsx";
import UserActivityPanel from "../../components/UserActivityPanel/UserActivityPanel.jsx";
import UserJoinedSynapsesPanel from "../../components/UserJoinedSynapsesPanel/UserJoinedSynapsesPanel.jsx";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUserByHandle from "../../api/hooks/useGetUserByHandle.js";

function useRootContext() {
    return useOutletContext(); // { user, localSynapseMetadata }
}

const UserProfileLayout = ({ children }) => {
    const { sessionUser, localSynapseMetadata } = useRootContext();
    const navigate = useNavigate(); // React Router navigate
    const { handle } = useParams();
    const [user, setUser] = useState(null)
    const [activeTab, setActiveTab] = useState("overview");
    const [isProfileOwner, setIsProfileOwner] = useState(false)
    const { getUserByHandle } = useGetUserByHandle();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();

    useEffect(() => {
        const checkProfileOwner = async () => {
            try {
                if (!handle) {
                    navigate(`/profile/${sessionUser.handle}`)
                }
                if (sessionUser.handle === handle) {
                    setIsProfileOwner(true)
                }
            } catch (error) {
                console.error("Error fetching current session user:", error);
                navigate('/login');
            }
        }
        checkProfileOwner();
    }, [handle])

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
        return <div className={'h-[100dvh] bg-background'}></div>;
    }

    return (
        <div className={'flex h-[100dvh] w-full items-center justify-center overflow-y-auto pt-16'}>
            {/* Remove the bg-background above to see the magic below */}
            {/*<div className={`absolute top-0 left-o -z-1 pt-17 h-screen w-screen bg-gradient-to-b from-background via-primary to-background backdrop-blur-lg `}/>*/}

            <div className='h-full flex flex-col w-full'>
                {/* Header */}
                <div className={`sticky top-0 z-50  shrink-0`}>
                    {/*<Header user={user} localSynapseMetadata={localSynapseMetadata}/>*/}
                </div>
                {/* IdentityPanel */}
                <div className="flex flex-col xl:flex-1 w-full xl:grid xl:grid-cols-12">
                    <div className={'flex flex-col flex-1 min-h-0 p-2 xl:p-4  w-full xl:col-span-4'}>
                        <IdentityPanel
                            user={user}
                            isProfileOwner={isProfileOwner}
                        />
                    </div>
                    {/* Main Content Grid */}
                    <div className='flex flex-col  w-full col-span-8'>
                        {/* Tab Switcher */}
                        <div className="flex flex-row bg-surface/70  justify-around border border-border p-2 m-4 text-sm xl:text-xl text-foreground font-montserrat shadows-2xl rounded-xl col-span-8">
                            <button
                                onClick={() => setActiveTab("overview")}
                                className={`${activeTab === "overview" ? "text-brand font-bold" : "hover:text-brand/60 hover:cursor-pointer"}`}
                            >
                                Overview
                            </button>
                            <button
                                onClick={() => setActiveTab("showcase")}
                                className={`${activeTab === "showcase" ? "text-brand font-bold" : "hover:text-brand/60 hover:cursor-pointer"}`}
                            >
                                Showcase
                            </button>
                            <button
                                onClick={() => setActiveTab("synapses")}
                                className={`${activeTab === "synapses" ? "text-brand font-bold" : "hover:text-brand/60 hover:cursor-pointer"}`}
                            >
                                Synapses
                            </button>
                            <button
                                onClick={() => setActiveTab("activity")}
                                className={`${activeTab === "activity" ? "text-brand font-bold" : "hover:text-brand/60 hover:cursor-pointer"}`}
                            >
                                Activity
                            </button>
                        </div>
                        <div className={'flex flex-col h-full m-2 xl:m-4'}>
                            {activeTab === "overview" ? (
                                <UserOverviewPanel
                                    user={user}
                                />
                            ) : activeTab === "showcase" ? (
                                <UserShowcasePanel/>
                            ) : activeTab === "activity" ? (
                                <UserActivityPanel user={user} localSynapseMetadata={localSynapseMetadata}/>
                            ) : activeTab === "synapses" ? (
                                <UserJoinedSynapsesPanel synapses={user.synapses} />
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