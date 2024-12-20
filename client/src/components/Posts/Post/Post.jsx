import "./Post.css";
import { useEffect, useState } from "react";
import { formatDate } from "../../../utils/dateUtils.js";
import useFollowActions from "../../../api/hooks/useFollowActions.js";
import {NavLink} from "react-router-dom";
import Comment from '../../Comments/Comment/Comment.jsx'

const Post = ({
                  user_id,
                  display_name,
                  handle,
                  date,
                  content,
                  likes,
                  comments,
                  onEdit,
                  onDelete,
                  isEditing,
                  editedContent,
                  onContentChange,
                  onSave,
              }) => {
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const [isFollowing, setIsFollowing] = useState(false);

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
            <div className="user-post__follow-actions">
                <button
                    className="user-post__follow-button"
                    onClick={isFollowing ? handleUnfollow : handleFollow}
                >
                    {isFollowing ? "Unfollow" : "Follow"}
                </button>
            </div>
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
            <div className="user-post__stats">
                <p className="user-post__stats-likes">{likes} likes</p>
                <p className="user-post__stats-comments">{comments} comments</p>
            </div>
            <div className="user-post__comments">

            </div>
        </div>
    );
};

export default Post;