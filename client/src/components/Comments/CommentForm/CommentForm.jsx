import { useState } from "react";
import useCreateComment from "../../../api/hooks/useCreateComment.js";
import useCreateNotification from "../../../api/hooks/useCreateNotification.js";

const CommentForm = ({
                         user_id,
                         session_user_id,
                         resource_type,
                         resource_id,
                         getComments,
                         setComments,
                         refreshComments,
                     }) => {
    const [text, setText] = useState("Add a comment...");
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);
    const actor_id = session_user_id;
    const action = "COMMENT";


    const { createComment, loading, error } = useCreateComment(() => refreshComments(resource_type, resource_id, getComments, setComments));
    const { createNotification } = useCreateNotification();

    const handleSubmit = async () => {
        const comment = {
            resource_type: resource_type,
            resource_id: resource_id,
            content: text,
        };

        const notification = {
            user_id: user_id,
            actor_id: actor_id,
            resource_type: resource_type,
            resource_id: resource_id,
            action: action,
        }

        console.log("Submitting comment:", comment);
        await createComment(comment);
        await createNotification(notification);
        setText(""); // Reset the text field after submission
    };

    const handleFormClick = () => {
        if (!formClicked && text === `Add a comment...`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="comment-form items-center text-center w-full lg:px-32 ">
            <div className={` `}
                onClick={handleFormClick}>
                <textarea
                    className="comment-form__entry-field p-4 mt-8  w-full rounded-2xl  bg-background"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="comment-form__button p-1 px-2 my-4 text-xs md:text-sm rounded-md bg-primary"
                    onClick={handleSubmit} disabled={loading}>
                {loading ? "Commenting..." : "Comment"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default CommentForm;