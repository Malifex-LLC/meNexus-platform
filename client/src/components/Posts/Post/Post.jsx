import "./Post.css";
import { useEffect, useState } from "react";
import { formatDate } from "../../../utils/dateUtils.js";
import useFollowActions from "../../../api/hooks/useFollowActions.js";
import {NavLink} from "react-router-dom";
import useGetComments from "../../../api/hooks/useGetComments.js"
import Comment from '../../Comments/Comment/Comment.jsx'
import CommentForm from '../../Comments/CommentForm/CommentForm.jsx'
import useEditComment from "../../../api/hooks/useEditComment.js"
import useDeleteComment from "../../../api/hooks/useDeleteComment.js";

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
    const resource_type = "post";

    const [isFollowing, setIsFollowing] = useState(false);
    const [comments, setComments] = useState([]);
    const [showComments, setShowComments] = useState(false);

    const {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
    } = useEditComment(() => refreshComments(resource_type, post_id, getComments, setComments));

    const { handleDeleteComment } = useDeleteComment(() => refreshComments(resource_type, post_id, getComments, setComments));

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", user_id);
        try {
            await followUser(user_id);
            setIsFollowing(true);
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

    return (
        <div className={`user-post ${isEditing ? "user-post--editing" : ""}`}>
            <div className="user-post__identity">
                <NavLink
                    className="user-post__display-name"
                    to={`/profile/${handle}`}
                >
                    {display_name}
                </NavLink>
                <NavLink
                    className="user-post__handle"
                    to={`/profile/${handle}`}
                >
                    @{handle}
                </NavLink>
                <div className="user-post__date">
                    <p>{formatDate(date)}</p>
                </div>
            </div>
            {!isOwner && (
                <div className="user-post__follow-actions">
                    <button
                        className="user-post__follow-button"
                        onClick={isFollowing ? handleUnfollow : handleFollow}
                    >
                        {isFollowing ? "Unfollow" : "Follow"}
                    </button>
                </div>
            )}
            <div className="user-post__content">
                {isEditing ? (
                    <textarea
                        className="user-post__textarea"
                        value={editedContent}
                        onChange={onContentChange}
                    />
                ) : (
                    <p>{content}</p>
                )}
            </div>
            {isOwner && (
                <div className="user-post__content-actions">
                    {isEditing ? (
                        <button
                            className="user-post__button user-post__button--save"
                            onClick={onSave}
                        >
                            Save
                        </button>
                    ) : (
                        <button
                            className="user-post__button user-post__button--edit"
                            onClick={onEdit}
                        >
                            Edit
                        </button>
                    )}
                    <button
                        className="user-post__button user-post__button--delete"
                        onClick={onDelete}
                    >
                        Delete
                    </button>
                </div>
            )}
            <div className="user-post__stats">
                <p className="user-post__stats-likes">{likes} likes</p>
                <p className={`user-post__stats-comments ${
                    comments.length > 0 ? "user-post__stats-comments--active" : ""
                    }`} onClick={toggleComments}>
                    {showComments ? "Hide Comments" : `${comments.length} Comments`}
                </p>
            </div>
            { showComments && (
                <div className="user-post__comments">
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