// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import useGetFollowerCount from "../../api/hooks/useGetFollowerCount.js";
import useGetFollowingCount from "../../api/hooks/useGetFollowingCount.js";
import IdentityQuickActionsPanel from "./IdentityQuickActionsPanel.jsx";
import StatusPanel from "./StatusPanel.jsx";

const IdentityPanel = ({user, isProfileOwner}) => {
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
        <div className={`relative flex flex-col p-4 h-full w-full bg-background rounded-xl border border-border shadow-2xl`}>
            {/* Top Profile Section */}
            <div className={'p-4'}>
                <div className={` p-4 flex flex-col text-foreground rounded-xl shadow-lg `}>
                    <div className={'p-4 bg-surface rounded-xl shadow-2xl'}>
                        <p className={`md:text-2xl xl:text-7xl`}>{user.displayName}</p>
                        <p className={`md:text-lg xl:text-5xl text-brand`}>@{user.handle}</p>
                    </div>
                </div>
            </div>
            <div className={'flex px-4 justify-center  rounded-xl'}>
                <img
                    className={`w-64 rounded-xl `}
                    src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                    alt={`${user.displayName}'s profile picture`}
                />
            </div>
            <div className={'flex flex-row items-center justify-center text-xl text-foreground p-4 mt-4 bg-surface rounded-xl shadow-lg'}>
                <div className={`flex flex-col items-center text-md`}>
                    <p className={`px-4 lg:px-2 xl:px-4 text-brand`}>{followerCount}</p>
                    <p className={`px-4 lg:px-2  xl:px-4`}>Followers</p>
                </div>
                <div className={`flex flex-col items-center text-md`}>
                    <p className={`px-4 lg:px-2 xl:px-4 text-brand`}>{followingCount}</p>
                    <p className={`px-4 lg:px-2 xl:px-4 `}>Following</p>
                </div>
            </div>
            {/* Status */}
            <div className={'p-4 rounded-xl shadow-lg'}>
                <StatusPanel />
            </div>
            <div className={'flex flex-col items-center'}>
                {/* Quick Actions */}
                {!isProfileOwner  ? (
                    <div className={'flex p-4 justify-evenly rounded-xl shadow-lg w-full'}>
                        <IdentityQuickActionsPanel/>
                    </div>
                ) : null}

            </div>
        </div>
    );
};

export default IdentityPanel;