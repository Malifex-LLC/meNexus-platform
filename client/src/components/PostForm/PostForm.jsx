import './PostForm.css';
import { useState } from "react";
import useCreatePost from '../../api/hooks/useCreatePost.js';


const PostForm = ({ handle, refreshPosts }) => {
    const [text, setText] = useState(`What's on your mind?`);
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createPost, loading, error } = useCreatePost(refreshPosts);

    const styles = {
        textarea: {
            width: expanded ? '43%' : '43%',
            height: expanded ? '200%' : '100%',
        },
    };

    const handleSubmit = async () => {
        const post = {
            handle: handle,
            content: text,
        };
        console.log("Submitting post:", post);
        await createPost(post);
        setText(""); // Reset the text field after submission
    };

    const handleFormClick = () => {
        if (!formClicked && text === `What's on your mind?`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="post-form">
            <div onClick={handleFormClick}>
                <textarea
                    className="post-form__entry-field"
                    style={styles.textarea}
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="post-form__button" onClick={handleSubmit} disabled={loading}>
                {loading ? "Posting..." : "Post"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default PostForm;
