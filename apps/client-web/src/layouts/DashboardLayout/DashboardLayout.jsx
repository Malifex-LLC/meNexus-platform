// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useState} from "react";
import Header from "../../components/Header/Header.jsx";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import { useSwipeable } from 'react-swipeable';
import JoinedSynapsesPanel from "../../components/JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import FollowedUsersPanel from "../../components/FollowedUsersPanel/FollowedUsersPanel.jsx";
import useGetUser from "../../api/hooks/useGetUser.js";
import { useNavigate } from "react-router-dom"
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import FeedPanel from "../../components/GlobalFeed/FeedPanel.jsx";
import GlobalActivityPanel from "../../components/GlobalActivityPanel/GlobalActivityPanel.jsx";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import SynapseMembersPanel from "../../components/SynapseMembersPanel/SynapseMembersPanel.jsx";
import SynapseActivityPanel from "../../components/SynapseActivityPanel/SynapseActivityPanel.jsx";

const DashboardLayout = ({ children }) => {
    const navigate = useNavigate(); // React Router navigate

    const [activePanel, setActivePanel] = useState(1); // 0: Social, 1: Main, 2: Activity
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActivePanel((prev) => Math.min(prev + 1, 2)),
        onSwipedRight: () => setActivePanel((prev) => Math.max(prev - 1, 0)),
    });

    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const [localSynapseMetadata, setLocalSynapseMetadata] = useState(null);
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();


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
        const getMetadata = async () => {
            try {
                const metadata = await getSynapseMetadata();
                setLocalSynapseMetadata(metadata);
            } catch (error) {
                console.error(error);
            }
        }
        getMetadata()
    }, [])


    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className='flex flex-col h-screen w-full'>
            <div className='sticky top-0 z-50 h-17 shrink-0 border-b border-border'>
                <Header
                    user={user}
                />
                {/* Mobile Nav */}
                <div className='flex  lg:hidden justify-around py-2 border-b border-border text-foreground'>
                    <button
                        onClick={() => setActivePanel(0)}
                        className={`${activePanel === 0 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Control Panel
                    </button>
                    <button
                        onClick={() => setActivePanel(1)}
                        className={`${activePanel === 1 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Synapses
                    </button>
                    <button
                        onClick={() => setActivePanel(2)}
                        className={`${activePanel === 2 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Following
                    </button>
                </div>
            </div>

            {/* DASHBOARD CONTENT AREA */}
            <div className="flex flex-1 w-full lg:grid lg:grid-cols-12 min-h-0 overflow-hidden" {...swipeHandlers}>
                {/* SIDEBAR — User Identity */}
                <div className={`
            ${activePanel === 2 ? 'flex' : 'hidden'}
            lg:flex flex-col flex-1 overflow-hidden min-h-0 p-4  w-full lg:col-span-4`}
                >
                    <ControlPanel user={user} />
                </div>

                {/* MAIN FEED — Core Content */}
                <div className={`
            ${activePanel === 0 ? 'flex' : 'hidden'}
            lg:flex flex-col flex-1 overflow-hidden p-4  w-full lg:col-span-5`}
                >
                    <FeedPanel />
                </div>

                {/* RIGHT RAIL — Synapse & Network Context */}
                <div className={`
            ${activePanel === 1 ? 'flex' : 'hidden'}
            lg:flex flex-col flex-1 w-full  lg:col-span-3 overflow-hidden p-4 `}
                >
                    <GlobalActivityPanel user={user} localSynapseMetadata={localSynapseMetadata} />
                </div>
            </div>


        </div>
    );
};

export default DashboardLayout;