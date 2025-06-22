import { formatDate } from "../../../utils/dateUtils.js";
import {NavLink} from "react-router-dom";
import {useEffect, useState} from "react";
import useGetUser from "../../../api/hooks/useGetUser.js";

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

    const { getUser, loading: userLoading, error: userError } = useGetUser();
    const [user, setUser] = useState(null);


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

    if (!user) {
        return (
            <div>Loading...</div>
        )
    }

    return (
        <div className={`user-post flex w-full p-4 lg:p-8 mb-16 rounded-xl bg-background text-foreground border ${isEditing ? "border-is-editing" : "border-transparent"}`}>
            <div className="user-comment__identity flex flex-col">
                {user.profilePicture ? (
                    <img
                        className="w-20 h-20 rounded-lg object-cover"
                        src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                        alt={`${user.displayName}'s profile picture`}
                    />
                ) : (
                    <div className="w-20 h-20 rounded-lg bg-muted">Loading...</div>
                )}
            </div>
            <div className="user-comment__content flex flex-col pl-6 w-full text-lg">
                <NavLink
                    className="text-md md:text-xl font-semibold hover:underline"
                    to={`/profile/${user.handle}`}
                >
                    {displayName}
                </NavLink>
                <NavLink
                    className="text-sm text-brand"
                    to={`/profile/${user.handle}`}
                >
                    @{user.handle}
                </NavLink>
                <div className="user-comment__date text-xs text-neutral">
                    <p>{formatDate(date)}</p>
                </div>
                <div className="mt-4">
                    {isEditing ? (
                        <textarea
                            className="user-comment__textarea w-full border border-border p-2"
                            value={editedContent}
                            onChange={onContentChange}
                        />
                    ) : (
                        <div className="text-md lg:text-3xl whitespace-pre-wrap">
                            <p>{content}</p>
                        </div>
                    )}
                </div>
            </div>
            {isOwner && (
                <div className="flex justify-end mt-4 ">
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