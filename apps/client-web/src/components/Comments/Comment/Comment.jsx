// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { formatDate } from "../../../utils/dateUtils.js";
import {NavLink} from "react-router-dom";
import React, {useEffect, useState} from "react";
import useGetUser from "../../../api/hooks/useGetUser.js";
import { HiDotsHorizontal } from "react-icons/hi";


const Comment = ({
    publicKey,
    sessionPublicKey,
    displayName,
    handle,
    date,
    content,
    isEditing,
    onEdit,
    onDelete,
    editedContent,
    onContentChange,
    onSave,
}) => {
    const isOwner = publicKey === sessionPublicKey;
    console.log("isOwner for comment: ", isOwner);

    const { getUser, loading: userLoading, error: userError } = useGetUser();
    const [user, setUser] = useState(null);
    const [showActions, setShowActions] = useState(false);



    useEffect(() => {
        const fetchUserData = async () => {
            try {
                const userData = await getUser(publicKey);
                console.log('Fetched userData:', userData);
                setUser(userData);
            } catch (error) {
                console.error("Error fetching userData: ", error);
            }
        }
        fetchUserData();
    }, [publicKey])

    const toggleActionsTray = () => {
        setShowActions((prevState) => !prevState);
    }

    // Close drawers on ESC
    useEffect(() => {
        if (!showActions) return;
        const onKey = (e) => e.key === 'Escape' && (setShowActions(false));
        window.addEventListener('keydown', onKey);
        return () => window.removeEventListener('keydown', onKey);
    }, [showActions]);

    if (!user) {
        return (
            <div>Loading...</div>
        )
    }

    return (
        <div className={`flex flex-col w-full p-2 pb-4 lg:p-4 lg:pb-8  rounded-xl bg-surface text-foreground border ${isEditing ? "border-is-editing" : "border-transparent"}`}>
            {/* ===== Mobile Backdrop (shared) ===== */}
            {(showActions) && (
                <button
                    className="fixed inset-0 z-[49] -mr-[100vw]  "
                    aria-label="Close menus"
                    onClick={() => { setShowActions(false); }}
                />
            )}
            <div className={`flex`}>
                <div className="flex">
                    {user.profilePicture ? (
                        <NavLink to={`/profile/${user.handle}`}>
                            <img
                                className="w-12 md:w-20 h-auto rounded-lg object-cover"
                                src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                                alt={`${user.displayName}'s profile picture`}
                            />
                        </NavLink>
                    ) : (
                        <div className="w-20 h-20 rounded-lg bg-muted">Loading...</div>
                    )}
                </div>
                <div className="flex flex-col pl-2 md:pl-4  w-full text-lg">
                    <NavLink
                        className="text-sm md:text-xl font-montserrat font-semibold hover:underline"
                        to={`/profile/${user.handle}`}
                    >
                        {displayName}
                    </NavLink>
                    <NavLink
                        className="text-xs md:text-lg text-brand font-jetbrains hover:underline hover:cursor-pointer"
                        to={`/profile/${user.handle}`}
                    >
                        @{user.handle}
                    </NavLink>
                    <div className="text-xs text-neutral font-montserrat">
                        <p>{formatDate(date)}</p>
                    </div>
                </div>
                {/* Actions */}
                <div className="relative inline-block text-left">
                    {/* Dots trigger */}
                    <button
                        onClick={toggleActionsTray}
                        className="inline-flex items-center justify-center md:p-2 rounded-lg hover:text-brand/50 hover:cursor-pointer text-xl md:text-3xl"
                        aria-haspopup="menu"
                        aria-expanded={showActions}
                    >
                        <HiDotsHorizontal />
                    </button>

                    {/* Actions tray */}
                    {showActions && isOwner && (
                        <div
                            role="menu"
                            className="absolute right-0 mt-2 w-44 origin-top-right rounded-xl border border-border bg-surface/70 backdrop-blur-xs shadow-xl ring-1 ring-border p-2 z-50"
                        >
                            <div className="gap-2">
                                {isEditing ? (
                                    <button
                                        onClick={onSave}
                                        className="w-full inline-flex items-center justify-center rounded-lg px-3 py-2 text-xs font-medium bg-save hover:bg-save/90 hover:cursor-pointer text-foreground"
                                        role="menuitem"
                                    >
                                        Save
                                    </button>
                                ) : (
                                    <button
                                        onClick={onEdit}
                                        className="w-full inline-flex items-center justify-center rounded-lg px-3 py-2 text-xs font-medium bg-edit hover:bg-edit-hover hover:cursor-pointer text-foreground"
                                        role="menuitem"
                                    >
                                        Edit
                                    </button>
                                )}

                                <button
                                    onClick={() => {
                                        onDelete();
                                        toggleActionsTray();
                                    }}
                                    className="w-full inline-flex items-center justify-center rounded-lg px-3 py-2 text-xs font-medium bg-delete hover:bg-delete-hover hover:cursor-pointer text-foreground "
                                    role="menuitem"
                                >
                                    Delete
                                </button>
                            </div>
                        </div>
                    )}
                </div>
            </div>
            {/* Content */}
            <div className="flex flex-col px-4">

                <div className={`flex w-full mt-4 font-inter`}>
                    {isEditing ? (
                        <textarea
                            className="flex w-full border border-border p-2 text-sm md:text-md lg:text-xl focus:outline-1 focus:outline-brand/60"
                            value={editedContent}
                            onChange={onContentChange}
                        />
                    ) : (
                        <div className="flex w-full text-md lg:text-2xl whitespace-pre-wrap">
                            <p>{content}</p>
                        </div>
                    )}
                </div>
            </div>

        </div>
    )
}

export default Comment;