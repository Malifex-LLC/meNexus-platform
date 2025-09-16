// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import useGetFollowerCount from "../../api/hooks/useGetFollowerCount.js";
import useGetFollowingCount from "../../api/hooks/useGetFollowingCount.js";
import IdentityQuickActionsPanel from "./IdentityQuickActionsPanel.jsx";
import useFollowActions from "../../api/hooks/useFollowActions.js";

const IdentityPanel = ({user, isProfileOwner}) => {

    const [isFollowing, setIsFollowing] = useState(false);
    const [followerCount, setFollowerCount] = useState(0);
    const [followingCount, setFollowingCount] = useState(0);

    const { followCheck } = useFollowActions();
    const { getFollowerCount, loading: followerCountLoading, error: followerCountError } = useGetFollowerCount();
    const { getFollowingCount, loading: followingCountLoading, error: followingCountError } = useGetFollowingCount();


    useEffect(() => {
        const fetchFollowStatus = async () => {
            try {
                const following = await followCheck(user.publicKey);
                setIsFollowing(following);
            } catch (error) {
                console.error("Error fetching follow status:", error);
            }
        }
        fetchFollowStatus();
    }, [])

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
        <div className={`flex flex-col p-4 w-full h-full bg-surface/70 rounded-xl border border-border shadow-lg`}>

            {/* Top Profile Section */}
            <div className={`z-1 flex flex-col w-full text-foreground rounded-xl shadow-lg `}>
                <div className={'p-4 bg-surface rounded-xl shadow-lg'}>
                    <p className={`text-3xl md:text-4xl xl:text-5xl font-montserrat`}>{user.displayName}</p>
                    <p className={`text-2xl md:text-3xl xl:text-4xl text-brand font-jetbrains`}>@{user.handle}</p>
                </div>
            </div>
            <div className={'relative flex w-full  p-4 justify-center'}>
                <img
                    className={`z-1 w-48 xl:w-64 rounded-xl`}
                    src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                    alt={`${user.displayName}'s profile picture`}
                />
                {/* Background gradient */}
                <div
                    aria-hidden
                    className="absolute z-0  inset-0 bg-gradient-to-r from-surface via-brand to-surface blur-xl"
                />
            </div>
            <div className={'z-1 flex flex-row w-full items-center justify-center text-xl text-foreground p-4 mt-4 bg-surface rounded-xl shadow-lg'}>
                <div className={`flex flex-col items-center text-md font-montserrat`}>
                    <p className={`px-4 lg:px-2 xl:px-4 text-brand`}>{followerCount}</p>
                    <p className={`px-4 lg:px-2  xl:px-4`}>Followers</p>
                </div>
                <div className={`flex flex-col items-center text-md font-montserrat`}>
                    <p className={`px-4 lg:px-2 xl:px-4 text-brand`}>{followingCount}</p>
                    <p className={`px-4 lg:px-2 xl:px-4`}>Following</p>
                </div>
            </div>
            {/* Status */}
            {/*<div className={'p-4 rounded-xl shadow-lg'}>*/}
            {/*    <StatusPanel />*/}
            {/*</div>*/}
            <div className={'flex flex-col w-full'}>
                {/* Quick Actions */}
                {!isProfileOwner  ? (
                    <div className={'flex w-full p-4'}>
                        <IdentityQuickActionsPanel
                            publicKey={user.publicKey}
                            isFollowing={isFollowing}
                            setIsFollowing={setIsFollowing}
                        />
                    </div>
                ) : null}

            </div>
        </div>
    );
};

export default IdentityPanel;