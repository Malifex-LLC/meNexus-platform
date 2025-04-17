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
import useGetProfile from "../../../api/hooks/useGetProfile.js";

const Post = ({
                  post_id,
                  user_id,
                  session_user_id,
                  display_name,
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
    const isOwner = user_id === session_user_id;
    console.log("isOwner for post: ", isOwner);

    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { getComments, commentsData } = useGetComments();
    const { createNotification } = useCreateNotification();
    const { handleDeleteComment } = useDeleteComment(() => refreshComments(resource_type, post_id, getComments, setComments));
    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();


    const {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
    } = useEditComment(() => refreshComments(resource_type, post_id, getComments, setComments));

    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [profile, setProfile] = useState({});

    const [isFollowing, setIsFollowing] = useState(false);
    const [comments, setComments] = useState([]);
    const [showComments, setShowComments] = useState(false);
    const resource_type = "POST";

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", user_id);
        const notification = {
            user_id: user_id,
            actor_id: session_user_id,
            resource_type: "FOLLOW",
            resource_id: session_user_id,
            action: "FOLLOW",
        }
        try {
            await followUser(user_id);
            setIsFollowing(true);
            await createNotification(notification);
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", user_id);
        try {
            await unfollowUser(user_id);
            setIsFollowing(false);
        } catch (err) {
            console.error('Error unfollowing user:', err);
        }
    };

    useEffect(() => {
        const fetchFollowStatus = async () => {
            try {
                const isCurrentlyFollowing = await followCheck(user_id);
                setIsFollowing(isCurrentlyFollowing);
            } catch (error) {
                console.error("Error fetching follow status:", error);
            }
        };

        fetchFollowStatus();
    }, [user_id]);

    useEffect(() => {
        const fetchComments = async () => {
            try {
                const newComments = await getComments(resource_type, post_id);
                setComments(newComments);

            } catch (err) {
                console.error("Error fetching comments for post_id: ", post_id, err);
            }
        }
        fetchComments(); // Only fetchComments if they are displayed
    },[])

    const toggleComments = () => {
        setShowComments((prev) => !prev); // Toggle visibility
    };

    // Fetch profile and posts once the current handle is determined
    useEffect(() => {
        setCurrentHandle(handle)
        if (currentHandle) {
            const fetchData = async () => {
                try {
                    console.log("Fetching profile and posts for handle:", currentHandle);
                    const [profileData] = await Promise.all([
                        getProfile(currentHandle),
                    ]);
                    console.log("Profile Data after getProfile() fetching is:", profileData);
                    setProfile(profileData);

                    const isCurrentlyFollowing = await followCheck(profileData.user_id);
                    console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                    setIsFollowing(isCurrentlyFollowing);
                } catch (error) {
                    console.error("Error fetching data:", error);
                }
            };

            fetchData();
        }
    }, [currentHandle]);


    return (
        <div className={`user-post ${isEditing ? "user-post--editing   border border-is-editing " : ""} 
        p-8 mb-16 rounded-xl bg-surface text-foreground`}>
            <div className={`flex gap-4`}>
                <img
                    className={`relative w-24 h-auto`}
                    src={profile.profile_picture}
                    alt={`${profile.display_name}'s profile picture`}
                />
                <div className="user-post__identity flex flex-col w-auto">

                    <Link
                        className="user-post__display-name text-2xl hover:cursor-pointer hover:underline"
                        to={`/profile/${handle}`}
                    >
                        {display_name}
                    </Link>
                    <Link
                        className="user-post__handle text-lg text-brand hover:cursor-pointer"
                        to={`/profile/${handle}`}
                    >
                        @{handle}
                    </Link>
                    <div className="user-post__date text-neutral">
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
            <div className="user-post__content pt-4">
                {isEditing ? (
                    <textarea
                        className="user-post__textarea w-full text-3xl"
                        value={editedContent}
                        onChange={onContentChange}
                    />
                ) : (
                    <div className={`text-3xl `}>
                        <p>{content}</p>
                    </div>
                )}
            </div>
            {isOwner && (
                <div className="user-post__content-actions flex justify-end gap-4 pt-4">
                    {isEditing ? (
                        <button
                            className="user-post__button user-post__button--save text-sm rounded-lg p-1 bg-save"
                            onClick={onSave}
                        >
                            Save
                        </button>
                    ) : (
                        <button
                            className="user-post__button user-post__button--edit text-sm rounded-lg p-1
                            bg-edit hover:bg-edit-hover"
                            onClick={onEdit}
                        >
                            Edit
                        </button>
                    )}
                    <button
                        className="user-post__button user-post__button--delete text-sm rounded-lg p-1
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
                <div className="user-post__comments mt-8">
                    {comments.length > 0 ? (
                        comments
                            .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                            .map((comment, index) => (
                                <Comment
                                    key={index}
                                    user_id={comment.comment_user_id}
                                    session_user_id={session_user_id}
                                    display_name={comment.display_name}
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
                </div>
            )}
            <div className="user-post__comment-form">
                <CommentForm
                    user_id={user_id}
                    session_user_id={session_user_id}
                    resource_type={resource_type}
                    resource_id={post_id}
                    getComments={getComments}
                    setComments={setComments}
                    refreshComments={refreshComments}
                />
            </div>
        </div>
    );
};

export default Post;