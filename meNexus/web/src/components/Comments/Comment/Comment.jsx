import { formatDate } from "../../../utils/dateUtils.js";
import {NavLink} from "react-router-dom";

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

    return (
        <div className={`user-comment relative flex flex-col p-4  mx-4 md:mx-16
        border border-border mb-4 bg-background text-sm ${isEditing ? "user-comment--editing" : ""}`}>
            <div className="user-comment__identity flex flex-col">
                <NavLink
                    className="user-comment__display-name"
                    to={`/profile/${handle}`}
                >
                    {displayName}
                </NavLink>
                <NavLink
                    className="user-comment__handle text-brand"
                    to={`/profile/${handle}`}
                >
                    @{handle}
                </NavLink>
                <div className="user-comment__date text-neutral">
                    <p>{formatDate(date)}</p>
                </div>

            </div>
            <div className="user-comment__content flex w-full mt-4 text-lg">
                {isEditing ? (
                    <textarea
                        className="user-comment__textarea w-full border border-border p-4"
                        value={editedContent}
                        onChange={onContentChange}
                    />
                ) : (
                    <p>{content}</p>
                )}
            </div>
            {isOwner && (
                <div className="flex justify-end gap-4 ">
                    {isEditing ?  (
                        <button
                            className="hover:underline"
                            onClick={onSave}
                        >
                            Save
                        </button>
                    ) : (
                        <button
                            className="hover:underline"
                            onClick={onEdit}
                        >
                            Edit
                        </button>
                    )}
                    <button
                        className="hover:underline"
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