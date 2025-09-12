// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ProfileCard from "../ProfileCard/ProfileCard.jsx";
import React, { useEffect, useState } from "react";
import useGetUser from "../../api/hooks/useGetUser.js";

const FollowedUsersPanel = ({following}) => {
    const { getUser } = useGetUser();

    const [allUsers, setAllUsers] = useState([])

    useEffect(() => {
        const fetchAllUsers = async () => {
            const usersList = await Promise.all(
                following.map(async (user) => {
                    return await getUser(user);
                })
            )
            setAllUsers(usersList)
        }
        fetchAllUsers();
    }, [])

    return (
        <div className={'flex flex-col w-full h-full  text-foreground text-center shadow-2xl'}>
            <div className="flex justify-around p-4 gap-4 bg-background border-b border-border text-2xl text-foreground shadows-2xl ">
                <button className={`text-brand font-bold `}>
                    Followed Users
                </button>
            </div>
            <div className={'p-4 text-left overflow-y-auto'}>
                {allUsers.map((user) => (
                    <ProfileCard
                        publicKey={user.publicKey}
                        handle={user.handle}
                        profilePicture={user.profilePicture}
                    />
                ))}
            </div>
        </div>
    );
}

export default FollowedUsersPanel