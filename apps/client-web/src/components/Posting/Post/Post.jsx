// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import { formatDate } from "../../../utils/dateUtils.js";
import useFollowActions from "../../../api/hooks/useFollowActions.js";
import { NavLink } from "react-router-dom";
import useGetComments from "../../../api/hooks/useGetComments.js"
import Comment from '../../Comments/Comment/Comment.jsx'
import CommentForm from '../../Comments/CommentForm/CommentForm.jsx'
import useEditComment from "../../../api/hooks/useEditComment.js"
import useDeleteComment from "../../../api/hooks/useDeleteComment.js";
import useCreateNotification from "../../../api/hooks/useCreateNotification.js"
import useGetUser from "../../../api/hooks/useGetUser.js";
import useFetchRemoteComments from "../../../api/hooks/useFetchRemoteComments.js";
import useUnfurlUrl from "../../../api/hooks/useUnfurlUrl.js";
import useEditRemoteComment from "../../../api/hooks/useEditRemoteComment.js";
import useDeleteRemotePostComment from "../../../api/hooks/useDeleteRemotePostComment.js";
import { HiDotsHorizontal } from "react-icons/hi";


const Post = ({
                  isLocalSynapse,
                  synapsePublicKey,
                  synapseUrl,
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
    const [showActions, setShowActions] = useState(false);
    const [comments, setComments] = useState([]);
    const [showComments, setShowComments] = useState(false);
    const [preview, setPreview] = useState(null);
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { getComments } = useGetComments();
    const { fetchRemoteComments } = useFetchRemoteComments();
    const { createNotification } = useCreateNotification();
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
        fetchComments(); // Only fetchComments if they are displayed
    },[postId])

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



    if (!user) {
        return
    }

    return (
        <div className={`grid grid-cols-12 w-full p-2 md:p-4   rounded-xl bg-background text-foreground border ${isEditing ? "border-is-editing" : "border-transparent"}`}>
            {/* Left Column: Profile */}
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
                {!isOwner && (
                    isFollowing ? (
                            <button
                                className="mt-4 text-xs px-2 py-1 rounded-md bg-surface/60 text-foreground hover:cursor-pointer hover:bg-brand/60"
                                onClick={isFollowing ? handleUnfollow : handleFollow}
                            >
                                Unfollow
                            </button>
                        ) : (
                            <button
                                className="mt-4 text-xs px-2 py-1 rounded-md bg-brand hover:cursor-pointer hover:bg-brand/60"
                                onClick={isFollowing ? handleUnfollow : handleFollow}
                            >
                                Follow
                            </button>
                        )
                )}
            </div>

            {/* Center Column: Content */}
            <div className="flex flex-col col-span-9 w-full flex-1 pl-6">
                {/* Identity */}
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
                    <div className="text-xs text-neutral font-montserrat">{formatDate(date)}</div>
                </div>

                {/* Post Content */}
                <div className="mt-4 font-inter">
                    {isEditing ? (
                        <textarea
                            className="w-full p-2 text-sm md:text-md lg:text-xl rounded-md bg-surface"
                            value={editedContent}
                            onChange={onContentChange}
                        />
                    ) : (
                        <p className="text-sm md:text-md lg:text-3xl whitespace-pre-wrap">{content}</p>
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
                            alt={`${postId}'s media`}
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

                {/* Stats */}
                <div className="mt-4 text-xs md:text-sm  text-neutral flex gap-4 font-montserrat">
                    <p>{likes} likes</p>
                    <p onClick={toggleComments} className="hover:underline cursor-pointer">
                        {showComments ? "Hide Comments" : `${comments.length} Comments`}
                    </p>
                </div>

            </div>

            {/* Right Column: Actions */}
            <div className="relative inline-block text-left">
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
                {showActions && isOwner && (
                    <div
                        role="menu"
                        className="absolute right-0 mt-2 w-44 origin-top-right rounded-xl border border-border bg-surface/70 backdrop-blur-xs shadow-xl ring-1 ring-border p-2 z-50"
                    >
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
                    </div>
                )}
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