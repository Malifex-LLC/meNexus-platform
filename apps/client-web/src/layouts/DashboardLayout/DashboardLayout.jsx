// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useEffect, useState} from "react";
import Header from "../../components/Header/Header.jsx";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import { useSwipeable } from 'react-swipeable';
import useGetUser from "../../api/hooks/useGetUser.js";
import { useNavigate } from "react-router-dom"
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import FeedPanel from "../../components/GlobalFeed/FeedPanel.jsx";
import GlobalActivityPanel from "../../components/GlobalActivityPanel/GlobalActivityPanel.jsx";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";

const DashboardLayout = ({ children }) => {
    const navigate = useNavigate(); // React Router navigate

    const [activePanel, setActivePanel] = useState(0); // 0: Control Panel, 1: Feed, 2: Activity
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
        <div className="flex flex-col h-[100dvh] w-full">
            {/* HEADER */}
            <div className="sticky top-0 z-50 shrink-0 border-b border-border bg-background ">
                <Header user={user} localSynapseMetadata={localSynapseMetadata}/>

                {/* Nav Tabs (Mobile) */}
                <div className="flex xl:hidden justify-around py-2 border-t border-border text-foreground mt-15">
                    <button
                        onClick={() => setActivePanel(0)}
                        className={`${activePanel === 0 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Control Panel
                    </button>
                    <button
                        onClick={() => setActivePanel(1)}
                        className={`${activePanel === 1 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Feed
                    </button>
                    <button
                        onClick={() => setActivePanel(2)}
                        className={`${activePanel === 2 ? 'text-brand font-semibold' : 'text-foreground hover:text-brand/60 hover:cursor-pointer'}`}
                    >
                        Activity
                    </button>
                </div>
            </div>

            {/* CONTENT */}
            <div
                className="flex flex-1 w-full min-h-0 overflow-hidden xl:grid xl:grid-cols-12 xl:gap-0 xl:mt-17"
                {...swipeHandlers}
            >
                {/* LEFT: Control Panel*/}
                <div
                    className={`${activePanel === 0 ? 'flex' : 'hidden'} xl:flex flex-col min-w-0 min-h-0 overflow-y-auto xl:p-4 w-full xl:col-span-4`}
                >
                    <ControlPanel user={user} />
                </div>

                {/* CENTER: Feed */}
                <div
                    className={`${activePanel === 1 ? 'flex' : 'hidden'} xl:flex flex-col min-w-0 min-h-0 overflow-y-auto xl:p-4 w-full xl:col-span-5`}
                >
                    <FeedPanel user={user} localSynapseMetadata={localSynapseMetadata}/>
                </div>

                {/* RIGHT: Global Activity */}
                <div
                    className={`${activePanel === 2 ? 'flex' : 'hidden'} xl:flex flex-col min-w-0 min-h-0 overflow-y-auto xl:p-4 w-full xl:col-span-3`}
                >
                    <GlobalActivityPanel user={user} localSynapseMetadata={localSynapseMetadata} />
                </div>
            </div>
        </div>
    );

};

export default DashboardLayout;