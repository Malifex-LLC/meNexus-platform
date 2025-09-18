// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import useCreateComment from "../../../api/hooks/useCreateComment.js";
import useCreateNotification from "../../../api/hooks/useCreateNotification.js";
import useCreateRemotePostComment from "../../../api/hooks/useCreateRemotePostComment.js";

const CommentForm = ({
                         isLocalSynapse,
                         publicKey,
                         synapsePublicKey,
                         sessionPublicKey,
                         resourceType,
                         resourceId,
                         setComments,
                         refreshComments,
                     }) => {
    const [text, setText] = useState("Add a comment...");
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);
    const actor_id = sessionPublicKey;
    const action = "COMMENT";


    const { createComment, loading, error } = useCreateComment(refreshComments);

    const { createRemotePostComment } = useCreateRemotePostComment(refreshComments);
    const { createNotification } = useCreateNotification();

    const handleSubmit = async () => {
        console.log('CommentForm handleSubmit isLocalSynapse: ', isLocalSynapse);
        if (isLocalSynapse) {
            const comment = {
                resourceType: resourceType,
                resourceId: resourceId,
                content: text,
                publicKey: actor_id,
            };

            const notification = {
                public_key: publicKey,
                actor_public_key: actor_id,
                resource_type: resourceType,
                resource_id: resourceId,
                action: action,
            }
            console.log("Submitting comment:", comment);
            await createComment(comment);
            //await createNotification(notification);
        } else {
            const comment = {
                resourceType: resourceType,
                resourceId: resourceId,
                content: text,
                publicKey: actor_id,
                synapsePublicKey: synapsePublicKey,
            }
            console.log("Submitting remote comment:", comment);
            await createRemotePostComment(comment);
            // TODO Create Remote Notification

        }
        setText(""); // Reset the text field after submission
    };

    const handleFormClick = () => {
        if (!formClicked && text === `Add a comment...`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="flex flex-col items-center text-center w-full  ">
            <div className={`flex w-full`}
                onClick={handleFormClick}>
                <textarea
                    className="flex w-full p-4 mt-8 rounded-xl bg-surface font-inter focus:outline-1 focus:outline-brand/60"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault(); // Prevent newline
                            handleSubmit();
                        }
                    }}
                />
            </div>
            <button className="p-1 px-2 my-4 text-xs md:text-sm rounded-md bg-primary"
                    onClick={handleSubmit} disabled={loading}>
                {loading ? "Commenting..." : "Comment"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
};

export default CommentForm;