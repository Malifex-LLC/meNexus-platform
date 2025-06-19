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

const Post = ({
                  postId,
                  publicKey,
                  sessionPublicKey,
                  displayName,
                  handle,
                  date,
                  content,
                  likes,
                  onEdit,
                  onDelete,
                  isEditing,
                  editedContent,
                  onContentChange,
                  onSave,
                  refreshComments,
              }) => {
    const isOwner = sessionPublicKey && publicKey === sessionPublicKey;

    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { getComments, commentsData } = useGetComments();
    const { createNotification } = useCreateNotification();
    const { handleDeleteComment } = useDeleteComment(() => refreshComments(resource_type, postId, getComments, setComments));
    const { getUser, loading: userLoading, error: userError } = useGetUser();

    const {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
    } = useEditComment(() => refreshComments(resource_type, postId, getComments, setComments));

    const [user, setUser] = useState(null);
    const [sessionUserPublicKey, setsessionUserPublicKey] = useState(null);
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [profilePicture, setProfilePicture] = useState('');


    const [isFollowing, setIsFollowing] = useState(false);
    const [comments, setComments] = useState([]);
    const [showComments, setShowComments] = useState(false);
    const resource_type = "POST";

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
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", publicKey);
        try {
            await unfollowUser(publicKey);
            setIsFollowing(false);
        } catch (err) {
            console.error('Error unfollowing user:', err);
        }
    };

    useEffect(() => {
        const fetchUserData = async () => {
            try {
                const userData = await getUser(publicKey);
                setUser(userData);
                setProfilePicture(userData.profilePicture);
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
            try {
                const newComments = await getComments(resource_type, postId);
                setComments(newComments);
            } catch (err) {
                console.error("Error fetching comments for postId: ", postId, err);
            }
        }
        fetchComments(); // Only fetchComments if they are displayed
    },[])

    const toggleComments = () => {
        setShowComments((prev) => !prev); // Toggle visibility
    };

    return (
        <div className={`user-post w-full ${isEditing ? "user-post--editing   border border-is-editing " : ""} 
        p-4 lg:p-8 mb-16 w-full rounded-xl bg-surface text-foreground`}>
            <div className={`flex  gap-4`}>
                <img
                    className={`relative w-16 h-16 md:w-24 md:h-auto`}
                    src={`${import.meta.env.VITE_API_BASE_URL}${profilePicture}`}
                    alt={`${displayName}'s profile picture`}
                />
                <div className="user-post__identity flex flex-col w-auto">
                    <Link
                        className="user-post__display-name text-md md:text-2xl hover:cursor-pointer hover:underline"
                        to={`/profile/${handle}`}
                    >
                        {displayName}
                    </Link>
                    <Link
                        className="user-post__handle text-sm md:text-xl text-brand hover:cursor-pointer"
                        to={`/profile/${handle}`}
                    >
                        @{handle}
                    </Link>
                    <div className="user-post__date text-xs md:text-lg text-neutral">
                        <p>{formatDate(date)}</p>
                    </div>
                </div>
            </div>
            {!isOwner && (
                <div className="user-post__follow-actions pt-4 pl-4">
                    <button
                        className="user-post__follow-button p-1 text-sm rounded-lg bg-brand"
                        onClick={isFollowing ? handleUnfollow : handleFollow}
                    >
                        {isFollowing ? "Unfollow" : "Follow"}
                    </button>
                </div>
            )}
            <div className="user-post__content pt-4 w-full">
                {isEditing ? (
                    <textarea
                        className="user-post__textarea w-full text-md lg:text-3xl"
                        value={editedContent}
                        onChange={onContentChange}
                    />
                ) : (
                    <div className={`text-md lg:text-3xl `}>
                        <p>{content}</p>
                    </div>
                )}
            </div>
            {isOwner && (
                <div className="user-post__content-actions flex justify-end gap-4 pt-4">
                    {isEditing ? (
                        <button
                            className="user-post__button user-post__button--save text-xs md:text-sm rounded-lg p-1 bg-save"
                            onClick={onSave}
                        >
                            Save
                        </button>
                    ) : (
                        <button
                            className="user-post__button user-post__button--edit text-xs md:text-sm rounded-lg p-1
                            bg-edit hover:bg-edit-hover"
                            onClick={onEdit}
                        >
                            Edit
                        </button>
                    )}
                    <button
                        className="user-post__button user-post__button--delete text-xs md:text-sm rounded-lg p-1
                        bg-delete hover:bg-delete-hover"
                        onClick={onDelete}
                    >
                        Delete
                    </button>
                </div>
            )}
            <div className="user-post__stats flex gap-4 text-neutral">
                <p className="user-post__stats-likes">{likes} likes</p>
                <p className={`user-post__stats-comments hover:underline ${
                    comments.length > 0 ? "user-post__stats-comments--active" : ""
                    }`} onClick={toggleComments}>
                    {showComments ? "Hide Comments" : `${comments.length} Comments`}
                </p>
            </div>
            { showComments && (
                <div className="user-post__comments mt-8 ">
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
                                    onContentChange={(event) =>
                                        setEditedCommentContent(event.target.value)
                                    }
                                    onSave={handleCommentSave}
                                />
                            ))
                    ) : (
                        <div>No comments</div>
                    )}
                    <div className="user-post__comment-form w-full">
                        <CommentForm
                            publicKey={publicKey}
                            sessionPublicKey={sessionPublicKey}
                            resource_type={resource_type}
                            resource_id={postId}
                            getComments={getComments}
                            setComments={setComments}
                            refreshComments={refreshComments}
                        />
                    </div>
                </div>
            )}

        </div>
    );
};

export default Post;