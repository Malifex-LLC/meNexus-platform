// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import useCreatePost from '../../../api/hooks/useCreatePost.js';
import useCreateRemotePost from "../../../api/hooks/useCreateRemotePost.js";

const PostForm = ({isLocalSynapse, publicKey, synapsePublicKey, activeBoard, refreshPosts }) => {
    const [text, setText] = useState(`What's on your mind?`);
    const [formClicked, setFormClicked] = useState(false);

    const { createPost, loading, error } = useCreatePost(refreshPosts);
    const { createRemotePost } = useCreateRemotePost(refreshPosts);

    const handleSubmit = async () => {
        console.log('handleSubmit called.');
        console.log('isLocalSynapse', isLocalSynapse);
        console.log('synapsePublicKey', synapsePublicKey);
        if (isLocalSynapse) {
            const post = {
                publicKey: publicKey,
                activeBoard: activeBoard,
                content: text,
            };
            console.log("Submitting post:", post);
            await createPost(post);
            setText(""); // Reset the text field after submission
            refreshPosts();
        } else {
            const post = {
                publicKey: publicKey,
                activeBoard: activeBoard,
                content: text,
                synapsePublicKey: synapsePublicKey,
            };
            console.log("Submitting remote post:", post);
            await createRemotePost(post);
            setText(""); // Reset the text field after submission
            refreshPosts();
        }
    };

    const handleFormClick = () => {
        if (!formClicked && text === `What's on your mind?`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="post-form  text-center  ">
            <div className={``}
                onClick={handleFormClick}>
                <textarea
                    className="post-form__entry-field p-4 w-full bg-background text-foreground rounded-md"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="post-form__button mt-2 px-4 bg-brand rounded-lg" onClick={handleSubmit} disabled={loading}>
                {loading ? "Posting..." : "Post"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default PostForm;