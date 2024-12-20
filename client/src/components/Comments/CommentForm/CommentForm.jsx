import './CommentForm.css';
import { useState } from "react";
import useCreateComment from "../../../api/hooks/useCreateComment.js";

const CommentForm = ({handle, refreshComments}) => {
    const [text, setText] = useState("Add a comment...");
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createComment, loading, error } = useCreateComment(refreshComments);


    const styles = {
        textarea: {
            width: expanded ? '43%' : '43%',
            height: expanded ? '200%' : '100%',
        },
    };

    const handleSubmit = async () => {
        const comment = {
            handle: handle,
            content: text,
        };
        console.log("Submitting comment:", comment);
        await createComment(comment);
        setText(""); // Reset the text field after submission
    };

    const handleFormClick = () => {
        if (!formClicked && text === `What's on your mind?`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="comment-form">
            <div onClick={handleFormClick}>
                <textarea
                    className="comment-form__entry-field"
                    style={styles.textarea}
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="comment-form__button" onClick={handleSubmit} disabled={loading}>
                {loading ? "Commenting..." : "Comment"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default CommentForm;