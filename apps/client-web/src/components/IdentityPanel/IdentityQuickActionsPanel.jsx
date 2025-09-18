// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useFollowActions from "../../api/hooks/useFollowActions.js";

const IdentityQuickActionsPanel = ({ publicKey, isFollowing, setIsFollowing }) => {
    const { followUser, unfollowUser } = useFollowActions();

    const handleFollow = async () => {
        await followUser(publicKey);
        setIsFollowing(true);
    }

    const handleUnfollow = async () => {
        await unfollowUser(publicKey);
        setIsFollowing(false);
    }

    return (
        <div className={'flex w-full gap-4 justify-evenly text-foreground font-montserrat'}>
            {isFollowing ? (
                <button
                    className={`p-2 w-32 rounded-xl cursor-pointer shadow-lg bg-surface hover:bg-brand/60 hover:tranlsate-y-[-2px]`}
                    onClick={ handleUnfollow }
                >
                    Unfollow
                </button>
            ): (
                <button
                    className={`p-2 w-32 rounded-xl cursor-pointer shadow-lg bg-brand hover:bg-brand/60 hover:tranlsate-y-[-2px]`}
                    onClick={ handleFollow }
                >
                    Follow
                </button>
            )}
            <button
                className={`p-2 w-32 bg-surface rounded-xl cursor-pointer shadow-lg hover:bg-brand/60 hover:tranlsate-y-[-2px]`}
                onClick={() => {}}
            >
                DM
            </button>
            <button
                className={`p-2 w-32 bg-surface rounded-xl cursor-pointer shadow-lg hover:bg-brand/60 hover:tranlsate-y-[-2px]`}
                onClick={() => {}}
            >
                Invite
            </button>
        </div>
    )
}

export default IdentityQuickActionsPanel;