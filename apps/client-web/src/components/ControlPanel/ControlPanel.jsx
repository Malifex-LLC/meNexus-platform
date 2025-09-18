// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import FollowedUsersPanel from "../FollowedUsersPanel/FollowedUsersPanel.jsx";
import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";

const ControlPanel = ({user}) => {

    return (
        <div className={`flex flex-col  p-2 xl:p-4 w-full h-full gap-4 items-center shadow-2xl bg-surface/70 border border-border xl:rounded-xl`}>
            <div className="flex flex-col w-full h-full">
                <div className="flex flex-col h-full w-full overflow-hidden rounded-2xl border border-border shadow-lg">
                    <div className="relative flex flex-col items-center p-4 sm:p-6 md:p-8">
                        <div className={`relative justify-center flex w-full h-full`}>
                            {/* Profile Picture + Gradient */}
                            <div
                                aria-hidden
                                className="absolute px-40  z-0  inset-0 bg-gradient-to-r from-surface via-brand to-surface blur-xl"
                            />
                            {/* Avatar */}
                            <img
                                className="z-1 w-auto h-full max-w-32 sm:max-w-48 lg:max-w-96 xl:max-w-32 rounded-xl ring-4 ring-surface shadow-md object-cover"
                                src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                                alt={`${user.displayName}'s profile picture`}
                                loading="lazy"
                            />
                        </div>

                        {/* Info */}
                        <div className="flex flex-col z-1 w-full max-w-xl  items-center mt-4 xl:mt-8">
                            {/* User Info */}
                            <div className="flex flex-col w-full items-center justify-center">
                                <p className="font-montserrat text-foreground font-semibold text-3xl xl:text-4xl">
                                    {user.displayName}
                                </p>
                                <p className="font-jetbrains text-brand text-xl xl:text-2xl">
                                    @{user.handle}
                                </p>
                            </div>
                            <div className={`flex w-full justify-between text-xl lg:text-2xl`}>
                                <div className="text-center font-montserrat">
                                    <p className="text-brand font-semibold ">
                                        {user.followers.length}
                                    </p>
                                    <p className="text-foreground text-[10px]">
                                        Followers
                                    </p>
                                </div>
                                <div className="text-center font-montserrat">
                                    <p className="text-brand font-semibold">
                                        {user.following.length}
                                    </p>
                                    <p className="text-foreground text-[10px]">
                                        Following
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div className={'flex flex-col w-full h-full overflow-y-auto border border-border rounded-xl'}>
                <JoinedSynapsesPanel synapses={user.synapses} />
            </div>
            <div className={'flex flex-col w-full h-full overflow-y-auto border border-border rounded-xl'}>
                <FollowedUsersPanel following={user.following} />
            </div>
        </div>
    );
};

export default ControlPanel;