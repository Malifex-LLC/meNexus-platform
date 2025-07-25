// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import { formatDate } from "../../../utils/dateUtils.js";
import useFollowActions from "../../../api/hooks/useFollowActions.js";
import {Link} from "react-router-dom";
import useGetComments from "../../../api/hooks/useGetComments.js"
import Comment from '../../Comments/Comment/Comment.jsx'
import CommentForm from '../../Comments/CommentForm/CommentForm.jsx'
import useEditComment from "../../../api/hooks/useEditComment.js"
import useDeleteComment from "../../../api/hooks/useDeleteComment.js";
import useCreateNotification from "../../../api/hooks/useCreateNotification.js"
import useGetUser from "../../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../../api/hooks/useGetSessionUser.js";
import useFetchRemoteComments from "../../../api/hooks/useFetchRemoteComments.js";
import { refreshComments } from "../../../utils/apiUtils.js"
import useUnfurlUrl from "../../../api/hooks/useUnfurlUrl.js";

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

    const {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
    } = useEditComment(handleRefreshComments);

    const { handleDeleteComment } = useDeleteComment(handleRefreshComments);

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
        const urls = extractUrls(content);
        if (urls.length > 0) {
            unfurlUrl(urls[0]).then(data => {
                if (data) setPreview(data);
            });
        }
    }, [content])

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
        <div className={`user-post flex w-full p-4   rounded-xl bg-surface text-foreground border ${isEditing ? "border-is-editing" : "border-transparent"}`}>
            {/* Left Column: Profile */}
            <div className="flex flex-col items-center w-24 shrink-0">
                {user.profilePicture ? (
                    <img
                        className="w-16 h-16 rounded-lg object-cover"
                        src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                        alt={`${user.displayName}'s profile picture`}
                    />
                ) : (
                    <div className="w-20 h-20 rounded-lg bg-muted">Loading...</div>
                )}
                {!isOwner && (
                    <button
                        className="mt-4 text-xs px-1 rounded-md bg-brand text-black"
                        onClick={isFollowing ? handleUnfollow : handleFollow}
                    >
                        {isFollowing ? "Unfollow" : "Follow"}
                    </button>
                )}
            </div>

            {/* Right Column: Content */}
            <div className="flex flex-col w-full flex-1 pl-6">
                {/* Identity */}
                <div>
                    <Link
                        className="text-md md:text-md font-semibold hover:underline"
                        to={`/profile/${user.handle}`}
                    >
                        {user.displayName}
                    </Link>
                    <div className="text-sm text-brand">@{user.handle}</div>
                    <div className="text-xs text-neutral">{formatDate(date)}</div>
                </div>

                {/* Post Content */}
                <div className="mt-4">
                    {isEditing ? (
                        <textarea
                            className="w-full p-2 text-md lg:text-xl rounded-md bg-background"
                            value={editedContent}
                            onChange={onContentChange}
                        />
                    ) : (
                        <p className="text-md lg:text-3xl whitespace-pre-wrap">{content}</p>
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

                {/* Post Actions */}
                {isOwner && (
                    <div className="flex justify-end gap-2 mt-4">
                        {isEditing ? (
                            <button className="text-xs bg-save px-3 py-1 rounded" onClick={onSave}>Save</button>
                        ) : (
                            <button className="text-xs bg-edit hover:bg-edit-hover px-3 py-1 rounded" onClick={onEdit}>Edit</button>
                        )}
                        <button className="text-xs bg-delete hover:bg-delete-hover px-3 py-1 rounded" onClick={onDelete}>Delete</button>
                    </div>
                )}

                {/* Stats */}
                <div className="mt-4 text-sm text-neutral flex gap-4">
                    <p>{likes} likes</p>
                    <p onClick={toggleComments} className="hover:underline cursor-pointer">
                        {showComments ? "Hide Comments" : `${comments.length} Comments`}
                    </p>
                </div>

                {/* Comments */}
                {showComments && (
                    <div className="mt-4 space-y-4">
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
                )}
            </div>
        </div>


    );
};

export default Post;