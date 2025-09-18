// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import { useNavigate, useOutletContext } from "react-router-dom";
import { useSwipeable } from "react-swipeable";
import { TbLogout2 } from "react-icons/tb";
import ProfileSettings from "../../components/Settings/ProfileSettings/ProfileSettings.jsx";
import AccountSettings from "../../components/Settings/AccountSettings/AccountSettings.jsx";
import DisplaySettings from "../../components/Settings/DisplaySettings/DisplaySettings.jsx";

function useRootContext() {
    return useOutletContext(); // { user, localSynapseMetadata }
}

const SettingsLayout = ({ children }) => {
    const { user, localSynapseMetadata } = useRootContext();
    const navigate = useNavigate();
    const [activePanel, setActivePanel] = useState(0); // 0: Profile, 1: Account, 2: Display
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActivePanel((prev) => Math.min(prev + 1, 2)),
        onSwipedRight: () => setActivePanel((prev) => Math.max(prev - 1, 0)),
    });

    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className='flex flex-col h-[100dvh] sm:items-center xl:items-start  bg-background pt-16'>
            <div className='sticky top-0 z-50 border-b border-border w-full'>

                {/* Mobile Nav */}
                <div className='flex  w-full xl:hidden py-2 justify-evenly border-b bg-background border-border text-foreground'>
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
            <div className={`flex h-full w-full overflow-y-auto`}>
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