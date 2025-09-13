// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useGetProfile from "../../api/hooks/useGetProfile.js";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import useGetFollowerCount from "../../api/hooks/useGetFollowerCount.js";
import useGetFollowingCount from "../../api/hooks/useGetFollowingCount.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import FollowedUsersPanel from "../FollowedUsersPanel/FollowedUsersPanel.jsx";
import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import {IoLocationSharp} from "react-icons/io5";

const ControlPanel = ({user}) => {
    const { getFollowerCount, loading: followerCountLoading, error: followerCountError } = useGetFollowerCount();
    const { getFollowingCount, loading: followingCountLoading, error: followingCountError } = useGetFollowingCount();

    const [followerCount, setFollowerCount] = useState(0);
    const [followingCount, setFollowingCount] = useState(0);

    useEffect(() => {
        const fetchFollowerCount = async () => {
            try {
                const fetchedFollowerCount = await getFollowerCount(user.publicKey);
                setFollowerCount(fetchedFollowerCount.result);
            } catch (error) {
                console.error("Error fetching follower count:", error);
            }
        }
        fetchFollowerCount();
    }, []);

    useEffect(() => {
        const fetchFollowingCount = async () => {
            try {
                const fetchedFollowingCount = await getFollowingCount(user.publicKey);
                setFollowingCount(fetchedFollowingCount.result);
            } catch (error) {
                console.error("Error fetching following count:", error);
            }
        }
        fetchFollowingCount();
    }, []);


    return (
        <div className={`flex flex-col overflow-y-auto p-4 h-full gap-4 items-center shadow-2xl bg-surface/70 border border-border rounded-xl`}>
            <div className="flex flex-col  p-4 w-full h-full items-center overflow-hidden rounded-2xl border border-border shadow-lg bg-surface/70">
                {/* Header stripe with gradient */}
                <div className="flex flex-col  mt-4 border border-border rounded-xl w-full h-32 bg-gradient-to-r from-surface via-brand to-surface blur-xl">
                </div>
                {/* Avatar with glow + gradient ring */}
                <div className="flex flex-col -mt-32 p-4">
                    <div className="z-1">
                        <img
                            className="w-32  object-cover ring-4 ring-surface"
                            src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                            alt={`${user.displayName}'s profile picture`}
                        />
                    </div>
                </div>

                {/* Identity */}


                {/* Stats row */}
                <div className="flex justify-center mt-4 w-full text-center">
                    <div className="p-2 mx-8">
                        <p className="text-brand text-lg font-semibold">{followerCount}</p>
                        <p className="text-foreground text-xs text-muted-foreground">Followers</p>
                    </div>
                    <div className=" flex flex-col -mt-4 items-center text-center px-4">
                        <p className="text-foreground text-xl md:text-2xl xl:text-3xl font-semibold">{user.displayName}</p>
                        <p className="text-brand text-sm md:text-lg xl:text-xl">@{user.handle}</p>
                    </div>
                    <div className="p-2 mx-8">
                        <p className="text-brand text-lg font-semibold">{followingCount}</p>
                        <p className="text-foreground text-xs text-muted-foreground">Following</p>
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