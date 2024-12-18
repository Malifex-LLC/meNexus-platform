import "./Post.css";
import {formatDate} from "../../../utils/dateUtils.js";


const Post = ({
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
    return (
        <div className={`user-post ${isEditing ? "user-post--editing" : ""}`}>
            <div className="user-post__identity">
                <h3 className="user-post__display-name">{display_name}</h3>
                <h4 className="user-post__handle">@{handle}</h4>
                <div className="user-post__date">
                    <p>{formatDate(date)}</p>
                </div>
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
            <div className="user-post__stats">
                <p className="user-post__likes">{likes} likes</p>
                <p className="user-post__comments">{comments} comments</p>
            </div>
            <div className="user-post__actions">
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
        </div>
    );
};

export default Post;
