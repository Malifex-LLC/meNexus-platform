import { useState } from "react";
import useCreatePost from '../../../api/hooks/useCreatePost.js';

const PostForm = ({ handle, refreshPosts }) => {
    const [text, setText] = useState(`What's on your mind?`);
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createPost, loading, error } = useCreatePost(refreshPosts);

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
        <div className="post-form  text-center p-4 mt-4">
            <div className={``}
                onClick={handleFormClick}>
                <textarea
                    className="post-form__entry-field p-4 w-full bg-background text-foreground rounded-md"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="post-form__button mt-4 px-4 bg-brand rounded-lg" onClick={handleSubmit} disabled={loading}>
                {loading ? "Posting..." : "Post"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default PostForm;