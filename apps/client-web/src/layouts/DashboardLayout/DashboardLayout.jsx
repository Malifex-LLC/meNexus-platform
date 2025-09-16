// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import { useSwipeable } from 'react-swipeable';
import { useOutletContext } from "react-router-dom"
import FeedPanel from "../../components/GlobalFeed/FeedPanel.jsx";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import GlobalActivityPanel from "../../components/GlobalActivityPanel/GlobalActivityPanel.jsx";

function useRootContext() {
    return useOutletContext(); // { sessionUser, user, localSynapseMetadata }
}

const DashboardLayout = ({children }) => {
    const { user, localSynapseMetadata } = useRootContext();
    const [activePanel, setActivePanel] = useState(0); // 0: Control Panel, 1: Feed, 2: Activity
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActivePanel((prev) => Math.min(prev + 1, 2)),
        onSwipedRight: () => setActivePanel((prev) => Math.max(prev - 1, 0)),
    });

    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className="flex flex-col h-[100dvh] w-full pt-16">
            {/* HEADER */}
            <div className=" z-50 shrink-0 border-b border-border bg-background">

                {/* Nav Tabs (Mobile) */}
                <div className="flex xl:hidden justify-around py-2 border-t border-border text-foreground">
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
                className="flex flex-1 w-full min-h-0 overflow-hidden xl:grid xl:grid-cols-12 xl:gap-0 "
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