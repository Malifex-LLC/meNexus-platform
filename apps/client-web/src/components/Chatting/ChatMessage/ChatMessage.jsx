// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { Link } from "react-router-dom";
import { formatDate } from "../../../utils/dateUtils.js";
import {useEffect, useState} from "react";
import useGetUser from "../../../api/hooks/useGetUser.js";

const ChatMessage = ({ message, isOwner }) => {
    const [user, setUser] = useState(null);
    const { getUser, loading: userLoading, error: userError } = useGetUser();

    useEffect(() => {
        const fetchUserData = async () => {
            try {
                const userData = await getUser(message.public_key);
                console.log('Fetched userData:', userData);
                setUser(userData);
            } catch (error) {
                console.error("Error fetching userData: ", error);
            }
        }
        fetchUserData();
    }, [message])

    if (!user) {
        return
    }

    return (
        <div
            className={`
        flex items-end gap-2 shadow-2xl
        ${isOwner ? "flex-row-reverse" : "flex-row"}
      `}
        >
            {/* avatar */}
            <img
                src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                alt={`${message.displayName}'s avatar`}
                className="w-8 h-8 rounded-md object-cover shrink-0"
            />

            {/* bubble */}
            <div
                className={`
          relative max-w-md px-3 py-2 rounded-lg  leading-snug
          ${isOwner
                    ? "bg-brand text-foreground-message after:border-l-brand"
                    : "bg-surface text-foreground after:border-r-surface"}
          after:content-[''] after:absolute after:-bottom-1
          ${isOwner
                    ? "after:-right-1 after:border-l-8 after:border-t-8 after:border-l-transparent after:border-t-transparent"
                    : "after:-left-1  after:border-r-8 after:border-t-8 after:border-r-transparent after:border-t-transparent"}
        `}
            >
                <Link
                    to={`/profile/${message.handle}`}
                    className="text-md font-semibold hover:underline"
                >
                    {message.displayName}
                </Link>

                <p className="text-xl whitespace-pre-wrap break-words">{message.content}</p>

                <span
                    className={`block mt-1 text-[10px] opacity-60 ${
                        isOwner ? "text-right" : ""
                    }`}
                >
          {formatDate(message.created_at)}
        </span>
            </div>
        </div>
    );
};

export default ChatMessage;
