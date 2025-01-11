import './Comment.css';
import { formatDate } from "../../../utils/dateUtils.js";
import {NavLink} from "react-router-dom";

const Comment = ({
    user_id,
    session_user_id,
    display_name,
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
    const isOwner = user_id === session_user_id;
    console.log("isOwner for comment: ", isOwner);

    return (
        <div className={`user-comment ${isEditing ? "user-comment--editing" : ""}`}>
            <div className="user-comment__identity">
                <NavLink
                    className="user-comment__display-name"
                    to={`/profile/${handle}`}
                >
                    {display_name}
                </NavLink>
                <NavLink
                    className="user-comment__handle"
                    to={`/profile/${handle}`}
                >
                    @{handle}
                </NavLink>
                <div className="user-comment__date">
                    <p>{formatDate(date)}</p>
                </div>

            </div>
            <div className="user-comment__content">
                {isEditing ? (
                    <textarea
                        className="user-comment__textarea"
                        value={editedContent}
                        onChange={onContentChange}
                    />
                ) : (
                    <p>{content}</p>
                )}
            </div>
            {isOwner && (
                <div className="user-comment__content-actions">
                    {isEditing ? (
                        <button
                            className="user-comment__button user-comment__button--save"
                            onClick={onSave}
                        >
                            Save
                        </button>
                    ) : (
                        <button
                            className="user-comment__button user-comment__button--edit"
                            onClick={onEdit}
                        >
                            Edit
                        </button>
                    )}
                    <button
                        className="user-comment__button user-comment__button--delete"
                        onClick={onDelete}
                    >
                        Delete
                    </button>
                </div>
            )}
        </div>
    )
}

export default Comment;