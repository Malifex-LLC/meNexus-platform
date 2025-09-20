// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useMemo, useState} from "react";
import { formatDate } from "../../../utils/dateUtils.js";
import useFollowActions from "../../../api/hooks/useFollowActions.js";
import { NavLink } from "react-router-dom";
import useGetComments from "../../../api/hooks/useGetComments.js"
import Comment from '../../Comments/Comment/Comment.jsx'
import CommentForm from '../../Comments/CommentForm/CommentForm.jsx'
import useEditComment from "../../../api/hooks/useEditComment.js"
import useDeleteComment from "../../../api/hooks/useDeleteComment.js";
import useCreateNotification from "../../../api/hooks/useCreateNotification.js";
import useCreateReaction from "../../../api/hooks/useCreateReaction.js";
import useDeleteReaction from "../../../api/hooks/useDeleteReaction.js";
import useCreateRemoteReaction from "../../../api/hooks/useCreateRemoteReaction.js";
import useDeleteRemoteReaction from "../../../api/hooks/useDeleteRemoteReaction.js";
import useGetReactions from "../../../api/hooks/useGetReactions.js"
import useFetchRemoteReactions from "../../../api/hooks/useFetchRemoteReactions.js"
import useGetUser from "../../../api/hooks/useGetUser.js";
import useFetchRemoteComments from "../../../api/hooks/useFetchRemoteComments.js";
import useUnfurlUrl from "../../../api/hooks/useUnfurlUrl.js";
import useEditRemoteComment from "../../../api/hooks/useEditRemoteComment.js";
import useDeleteRemotePostComment from "../../../api/hooks/useDeleteRemotePostComment.js";
import { HiDotsHorizontal } from "react-icons/hi";
import { BsArrowUpSquare } from "react-icons/bs";
import {PiArrowFatUpBold} from "react-icons/pi";
import {AiFillThunderbolt, AiOutlineThunderbolt} from "react-icons/ai";
import {BiComment} from "react-icons/bi";
import {IoShareSocialOutline} from "react-icons/io5";



const Post = ({
                  mode,
                  isLocalSynapse,
                  synapsePublicKey,
                  synapseUrl,
                  synapseName,
                  postId,
                  publicKey,
                  sessionPublicKey,
                  date,
                  content,
                  mediaUrl,
                  likes,
                  onEdit,
                  onDelete,
                  isEditing,
                  editedContent,
                  onContentChange,
                  onSave,
              }) => {
    const isOwner = sessionPublicKey && publicKey === sessionPublicKey;
    const [user, setUser] = useState(null);
    const [isFollowing, setIsFollowing] = useState(false);
    const [isBoosted, setIsBoosted] = useState(false);
    const [showActions, setShowActions] = useState(false);
    const [comments, setComments] = useState([]);
    const [reactions, setReactions] = useState([]);
    const [showComments, setShowComments] = useState(false);
    const [preview, setPreview] = useState(null);
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { getComments } = useGetComments();
    const { getReactions } = useGetReactions();
    const { fetchRemoteReactions } = useFetchRemoteReactions();
    const { createRemoteReaction } = useCreateRemoteReaction();
    const { deleteRemoteReaction } = useDeleteRemoteReaction();
    const { fetchRemoteComments } = useFetchRemoteComments();
    const { createNotification } = useCreateNotification();
    const { createReaction } = useCreateReaction();
    const { deleteReaction } = useDeleteReaction();
    const { getUser, loading: userLoading, error: userError } = useGetUser();
    const { unfurlUrl } = useUnfurlUrl();

    const resource_type = "POST";

    const handleRefreshComments = async () => {
        if (isLocalSynapse) {
            const updatedComments = await getComments(resource_type, postId);
            setComments(updatedComments);
        } else {
            const updatedComments = await fetchRemoteComments(resource_type, postId, synapsePublicKey);
            setComments(updatedComments);
        }
    };

    const localEdit = useEditComment(handleRefreshComments);
    const remoteEdit = useEditRemoteComment(handleRefreshComments, synapsePublicKey);

    const {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
    } = isLocalSynapse ? localEdit : remoteEdit;

    const localDelete = useDeleteComment(handleRefreshComments);
    const remoteDelete = useDeleteRemotePostComment(handleRefreshComments, synapsePublicKey);

    const {
        handleDeleteComment
    } = isLocalSynapse ? localDelete : remoteDelete;

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", publicKey);
        const notification = {
            public_key: publicKey,
            actor_public_key: sessionPublicKey,
            resource_type: "FOLLOW",
            resource_id: sessionPublicKey,
            action: "FOLLOW",
        }
        try {
            await followUser(publicKey);
            setIsFollowing(true);
            await createNotification(notification);
        } catch (error) {
            console.log('Error following user', error);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", publicKey);
        try {
            await unfollowUser(publicKey);
            setIsFollowing(false);
        } catch (error) {
            console.error('Error unfollowing user:', error);
        }
    };

    const fetchReactions = async () => {
        if (isLocalSynapse) {
            try {
                const response = await getReactions(resource_type, postId);
                setReactions(response);
            } catch (error) {
                console.error("Error fetching reactions for postId: ", postId, error);
            }
        } else {
            try {
                const response = await fetchRemoteReactions(resource_type, postId, synapsePublicKey);
                setReactions(response);
            } catch (error) {
                console.error("Error fetching reactions for postId: ", postId, error);
            }
        }
    }

    const boosts = useMemo(
        () => reactions?.filter(reaction => reaction.reaction_type === 'BOOST'),
        [reactions]
    );

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

    useEffect(() => {
        const fetchFollowStatus = async () => {
            try {
                const isCurrentlyFollowing = await followCheck(publicKey);
                console.log('Post calling followCheck for followedPublicKey: ', publicKey);
                console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                setIsFollowing(isCurrentlyFollowing);
            } catch (error) {
                console.error("Error fetching follow status:", error);
            }
        };
        fetchFollowStatus();
    }, [publicKey]);

    useEffect(() => {
        const fetchComments = async () => {
            setShowComments(false);
            if (isLocalSynapse) {
                try {
                    const newComments = await getComments(resource_type, postId);
                    setComments(newComments);
                } catch (error) {
                    console.error("Error fetching comments for postId: ", postId, error);
                }
            } else {
                try {
                    const newComments = await fetchRemoteComments(resource_type, postId, synapsePublicKey);
                    setComments(newComments);
                } catch (error) {
                    console.error("Error fetching remote comments for postId: ", postId, error);
                }
            }
        }
        fetchComments();
    },[postId])

    useEffect(() => {
        setIsBoosted();
        fetchReactions();
    }, [postId])

    useEffect(() => {
        const boostCheck = async () => {
            if (!reactions || !reactions.length) return;
            reactions.map(reaction => {
                if (reaction.public_key === sessionPublicKey && reaction.reaction_type === 'BOOST') {
                    setIsBoosted(true);
                }
            })
        }
        boostCheck();
    }, [reactions])

    useEffect(() => {
        setPreview(null);

        const urls = extractUrls(content);
        if (urls.length > 0) {
            unfurlUrl(urls[0]).then(data => {
                if (data) setPreview(data);
            });
        }
    }, [postId, content])

    const toggleActionsTray = () => {
        setShowActions((prevState) => !prevState);
    }

    const toggleComments = () => {
        setShowComments((prev) => !prev); // Toggle visibility
    };

    const handleBoost = async () => {
        if (isLocalSynapse) {
            if (!isBoosted) {
                try {
                    await createReaction(sessionPublicKey, postId, resource_type, 'BOOST')
                    await fetchReactions()
                    setIsBoosted(true)
                } catch (error) {
                    console.error("Error creating reaction boost: ", error);
                }
            } else {
                try {
                    await deleteReaction(sessionPublicKey, postId, resource_type, 'BOOST');
                    await fetchReactions()
                    setIsBoosted(false);
                } catch (error) {
                    console.error("Error deleting reaction boost: ", error);
                }
            }
        } else {
            if (!isBoosted) {
                try {
                    await createRemoteReaction(sessionPublicKey, postId, resource_type, 'BOOST', synapsePublicKey)
                    await fetchReactions();
                    setIsBoosted(true)
                } catch (error) {
                    console.error("Error creating remote reaction boost: ", error);
                }
            } else {
                try {
                    await deleteRemoteReaction(sessionPublicKey, postId, resource_type, 'BOOST', synapsePublicKey);
                    await fetchReactions();
                    setIsBoosted(false);
                } catch (error) {
                    console.error("Error deleting remote reaction boost: ", error);
                }
            }
        }

    }

    const getMediaTypeFromUrl = (url) =>  {
        const extension = url.split('.').pop().toLowerCase();
        const imageExtensions = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp'];
        const videoExtensions = ['mp4', 'webm', 'ogg', 'mov'];

        if (imageExtensions.includes(extension)) return 'image';
        if (videoExtensions.includes(extension)) return 'video';
        return 'unknown';
    }

    const urlRegex = /https?:\/\/[^\s]+/g;

    const extractUrls = (text) => {
        const matches = text.match(urlRegex);
        return matches || [];
    };

    // Close drawers on ESC
    useEffect(() => {
        if (!showActions) return;
        const onKey = (e) => e.key === 'Escape' && (setShowActions(false));
        window.addEventListener('keydown', onKey);
        return () => window.removeEventListener('keydown', onKey);
    }, [showActions]);

    if (!user) {
        return
    }

    return (
        <div className={`flex flex-col w-full p-2 md:p-4   rounded-xl bg-background text-foreground border ${isEditing ? "border-is-editing" : "border-transparent"}`}>
            {/* ===== Mobile Backdrop (shared) ===== */}
            {(showActions) && (
                <button
                    className="fixed inset-0 z-[49] -mr-[100vw]  "
                    aria-label="Close menus"
                    onClick={() => { setShowActions(false); }}
                />
            )}
            {mode === 'GLOBAL' && (
                <div className={`flex w-full text-sm xl:text-md justify-end text-neutral gap-1 mb-2`}>
                    Posted in
                    <NavLink
                        className={`text-accent hover:text-accent/60 hover:underline`}
                        to={`/synapse/${synapsePublicKey}`}
                    >
                        {synapseName}
                    </NavLink>
                </div>
                )}
            {/* Profile */}
            <div className={`flex `}>
                <div className="flex flex-col col-span-2 items-center w-16 md:w-24 shrink-0">
                    {user.profilePicture ? (
                        <NavLink to={`/profile/${user.handle}`}>
                            <img
                                className="w-16 md:w-24 rounded-lg object-cover"
                                src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                                alt={`${user.displayName}'s profile picture`}
                            />
                        </NavLink>
                    ) : (
                        <div className="w-20 h-20 rounded-lg bg-muted">Loading...</div>
                    )}
                </div>

                {/* Metadata */}
                <div className="flex flex-col w-full flex-1 pl-4">
                    <div className={`flex flex-col`}>
                        <NavLink
                            className="text-sm md:text-xl font-montserrat font-semibold hover:underline hover:pointer-cursor"
                            to={`/profile/${user.handle}`}
                        >
                            {user.displayName}
                        </NavLink>
                        <NavLink
                            className={`text-xs md:text-lg text-brand font-jetbrains hover:underline hover:pointer-cursor`}
                            to={`/profile/${user.handle}`}>
                            @{user.handle}
                        </NavLink>
                        <div className="text-xs text-neutral font-inter">{formatDate(date)}</div>
                    </div>
                </div>
                {/* Actions */}
                <div className="relative inline-block">
                    {/* Dots trigger */}
                    <button
                        onClick={toggleActionsTray}
                        className="inline-flex items-center justify-center md:p-2 rounded-lg hover:text-brand/50 hover:cursor-pointer text-2xl md:text-3xl"
                        aria-haspopup="menu"
                        aria-expanded={showActions}
                    >
                        <HiDotsHorizontal />
                    </button>

                    {/* Actions tray */}
                    {showActions && (
                        <div
                            role="menu"
                            className="absolute right-0 mt-2 w-44 origin-top-right rounded-xl border border-border bg-surface/70 backdrop-blur-xs shadow-xl ring-1 ring-border p-2 z-50"
                        >
                            {isOwner && (
                                <div className="grid gap-2">
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
                            )}
                            {!isOwner && (
                                isFollowing ? (
                                    <button
                                        className="w-full inline-flex items-center justify-center rounded-lg px-3 py-2 text-xs font-medium bg-background hover:bg-brand/60 hover:cursor-pointer text-foreground"
                                        onClick={isFollowing ? handleUnfollow : handleFollow}
                                    >
                                        Unfollow
                                    </button>
                                ) : (
                                    <button
                                        className="w-full inline-flex items-center justify-center rounded-lg px-3 py-2 text-xs font-medium bg-brand hover:bg-brand/60 hover:cursor-pointer text-foreground"
                                        onClick={isFollowing ? handleUnfollow : handleFollow}
                                    >
                                        Follow
                                    </button>
                                )
                            )}
                        </div>
                    )}
                </div>
            </div>
            {/* Post Content */}
            <div className={`px-4`}>
                <div className="w-full mt-4 font-inter">
                    {isEditing ? (
                        <textarea
                            className="w-full p-2 text-sm md:text-md lg:text-xl rounded-md bg-surface"
                            value={editedContent}
                            onChange={onContentChange}
                        />
                    ) : (
                        <p className="w-full text-md lg:text-2xl whitespace-pre-wrap">{content}</p>
                    )}
                </div>

                {/* Post Media */}
                {mediaUrl && getMediaTypeFromUrl(mediaUrl) === 'image' ? (
                    <div className="flex justify-center items-center mt-4">
                        <img
                            className="rounded-lg max-w-full h-auto object-contain"
                            src={`${isLocalSynapse ? import.meta.env.VITE_API_BASE_URL : synapseUrl}${mediaUrl}`}
                            alt={`${postId}'s media`}
                        />
                    </div>
                ) : mediaUrl && getMediaTypeFromUrl(mediaUrl) === 'video' ? (
                    <div className="flex justify-center items-center mt-4">
                        <video
                            className="rounded-lg max-w-full h-auto object-contain"
                            src={`${isLocalSynapse ? import.meta.env.VITE_API_BASE_URL : synapseUrl}${mediaUrl}`}
                            controls={true}
                        />
                    </div>
                ) : null
                }

                {/* Link Previews */}
                {preview && (
                    <div className="p-4 mt-4 w-full border border-border rounded-xl shadow-xl ">
                        <a href={preview.url} target="_blank" className="text-lg font-bold text-brand"><img src={preview.image} alt="" className="rounded-xl mb-2" /></a>
                        <a href={preview.url} target="_blank" className="text-lg font-bold text-brand">{preview.title}</a>
                        <p className="text-sm text-foreground">{preview.description}</p>
                    </div>

                )}
            </div>


            {/* Stats */}
            <div className={`flex flex-col border-t border-border mt-8`}>
                <div className={`flex w-full justify-between mt-4 px-4 text-xs md:text-sm  text-neutral  gap-4 font-montserrat`}>
                    <p>{boosts.length} Boosts</p>
                    <p onClick={toggleComments} className="hover:underline cursor-pointer">
                        {showComments ? "Hide Comments" : `${comments.length} Comments`}
                    </p>
                </div>
                <div className={`flex w-full gap-4 text-2xl xl:text-2xl justify-evenly px-4 py-2 mt-2`}>
                    <button
                        className={`hover:cursor-pointer hover:bg-brand/60 active:scale-90 rounded-sm p-2 
                        `}
                        onClick={handleBoost}
                    >
                        {isBoosted ? (
                            <div className={`text-brand`}>
                                <AiFillThunderbolt />
                            </div>
                        ) : (
                            <AiOutlineThunderbolt />
                        )}
                    </button>
                    <button
                        className={'hover:cursor-pointer hover:bg-brand/60 active:scale-90 rounded-sm p-2'}
                        onClick={toggleComments}
                    >
                        <BiComment />
                    </button>
                    <button className={'hover:cursor-pointer hover:bg-brand/60 active:scale-90 rounded-sm p-2'}>
                        <IoShareSocialOutline />
                    </button>
                </div>
            </div>
            {/* Comments */}
            {showComments && (
                <div className="flex flex-col col-start-3 col-span-10 w-full mt-4 md:p-4 space-y-4">
                    {comments.length > 0 ? (
                        comments
                            .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                            .map((comment, index) => (
                                <Comment
                                    key={index}
                                    publicKey={comment.comment_public_key}
                                    sessionPublicKey={sessionPublicKey}
                                    displayName={comment.displayName}
                                    handle={comment.handle}
                                    date={comment.comment_created_at}
                                    content={comment.comment_content}
                                    isEditing={editingCommentId === comment.comment_id}
                                    onEdit={() => handleCommentEdit(comment.comment_id, comments)}
                                    onDelete={() => handleDeleteComment(comment.comment_id)}
                                    editedContent={editedCommentContent}
                                    onContentChange={(e) => setEditedCommentContent(e.target.value)}
                                    onSave={handleCommentSave}
                                />
                            ))
                    ) : (
                        <div>No comments</div>
                    )}
                    <div className={`flex w-full`}>
                        <CommentForm
                            isLocalSynapse={isLocalSynapse}
                            publicKey={publicKey}
                            synapsePublicKey={synapsePublicKey}
                            sessionPublicKey={sessionPublicKey}
                            resourceType="POST"
                            resourceId={postId}
                            setComments={setComments}
                            refreshComments={handleRefreshComments}
                        />
                    </div>
                </div>
            )}

        </div>


    );
};

export default Post;