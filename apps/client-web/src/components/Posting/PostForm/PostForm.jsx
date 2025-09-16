// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import useCreatePost from '../../../api/hooks/useCreatePost.js';
import useCreateRemotePost from "../../../api/hooks/useCreateRemotePost.js";
import useUploadPostMedia from "../../../api/hooks/useUploadPostMedia.js";
import { IoMdAttach } from "react-icons/io";
import useUploadRemotePostMedia from "../../../api/hooks/useUploadRemotePostMedia.js";


const PostForm = ({isLocalSynapse, publicKey, synapsePublicKey, activeBoard, refreshPosts }) => {
    const [text, setText] = useState(`What's on your mind?`);
    const [formClicked, setFormClicked] = useState(false);
    const [selectedFile, setSelectedFile] = useState(null);
    const [previewUrl, setPreviewUrl] = useState(null);
    const [ isUploadSuccess, setIsUploadSuccess ] = useState(false);
    const [ isUploadError, setIsUploadError ] = useState(false);
    const [ postId, setPostId ] = useState(null);

    const { createPost, loading, error } = useCreatePost(refreshPosts);
    const { createRemotePost } = useCreateRemotePost(refreshPosts);
    const { uploadPostMedia, loading: uploadPostMediaLoading, error: uploadPostMediaError } = useUploadPostMedia();
    const { uploadRemotePostMedia } = useUploadRemotePostMedia();
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
            const response = await createPost(post);
            console.log("createPost response: ", response);
            setText(""); // Reset the text field after submission
            setPostId(response.data.postId)
            if (selectedFile) {
                console.log("Uploading post media for: ", selectedFile.name, ", ", response.data.postId, ", ", publicKey);
                await handleUploadPostMedia(selectedFile, response.data.postId, publicKey)
            }
            refreshPosts();
        } else {
            const post = {
                publicKey: publicKey,
                activeBoard: activeBoard,
                content: text,
                synapsePublicKey: synapsePublicKey,
            };
            console.log("Submitting remote post:", post);
            const response = await createRemotePost(post);
            console.log("create Remote post response: ", response);
            setText(""); // Reset the text field after submission
            setPostId(response.data.response.payload.post.postId)
            if (selectedFile) {
                console.log("Uploading remote post media for: ", selectedFile.name, ", ", response.data.response.payload.post.postId, ", ", publicKey);
                await handleUploadRemotePostMedia(selectedFile, response.data.response.payload.post.postId, publicKey, synapsePublicKey)
            }
            refreshPosts();
        }
    };

    const handleFormClick = () => {
        if (!formClicked && text === `What's on your mind?`) {
            setText("");
        }
        setFormClicked(true);
    };

    const handleFileChange = (event) => {
        const file = event.target.files[0];
        setSelectedFile(file);

        if (file) {
            const preview = URL.createObjectURL(file);
            setPreviewUrl(preview);
            console.log('Preview URL: ', preview);
        } else {
            setPreviewUrl(null);
        }
    };


    const handleUploadPostMedia = async (selectedFile, postId, publicKey) => {
        setIsUploadError(false);
        setIsUploadSuccess(false);

        if (!selectedFile) {
            console.log('Please select a file to upload.');
            return;
        }

        try {
            const response = await uploadPostMedia(selectedFile, postId, publicKey);

            if(response.status === 200) {
                console.log('Post media uploaded successfully!');
                setIsUploadSuccess(true);
                setPreviewUrl(null)
                setSelectedFile(null);
            }
        } catch (error) {
            console.error('Error uploading post media:', error.message);
            console.log('Failed to upload post media.');
            setIsUploadError(true);
        }
    };

    const handleUploadRemotePostMedia = async (selectedFile, postId, publicKey, synapsePublicKey) => {
        setIsUploadError(false);
        setIsUploadSuccess(false);

        if (!selectedFile) {
            console.log('Please select a file to upload.');
            return;
        }

        try {
            const response = await uploadRemotePostMedia(selectedFile, postId, publicKey, synapsePublicKey);

            if(response.status === 200) {
                console.log('Post media uploaded successfully!');
                setIsUploadSuccess(true);
                setPreviewUrl(null)
                setSelectedFile(null);
            }
        } catch (error) {
            console.error('Error uploading post media:', error.message);
            console.log('Failed to upload post media.');
            setIsUploadError(true);
        }
    };

    return (
        <div className="flex flex-col post-form text-center">
            {/* Textarea Input */}
            <div onClick={handleFormClick}>
        <textarea
            className="post-form__entry-field p-4 w-full bg-surface text-foreground rounded-md"
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

            {/* Button + Attach Icon */}
            <div className="flex items-center justify-center gap-4 mt-4">
                <button
                    className="post-form__button px-4 py-2 bg-brand hover:bg-brand/60 text-white rounded-lg hover:cursor-pointer active:scale-90"
                    onClick={handleSubmit}

                    disabled={loading}
                >
                    {loading ? "Posting..." : "Post"}
                </button>

                <label
                    htmlFor="postMedia"
                    className="w-10 h-10 flex items-center justify-center bg-surface/30 hover:bg-brand/60 active:bg-surface active:scale-90 text-white text-2xl rounded-xl border border-border cursor-pointer transition-colors duration-150"
                    title="Attach media"
                >
                    <IoMdAttach />
                </label>

                <input
                    id="postMedia"
                    type="file"
                    accept="image/*, video/*"
                    onChange={handleFileChange}
                    className="hidden"
                />
            </div>

            {/* Selected File */}
            {selectedFile && (
                <span className="mt-2 text-sm text-foreground">
            Selected: {selectedFile.name}
        </span>
            )}

            {/* Preview Image */}
            {previewUrl && (
                <img
                    src={previewUrl}
                    alt="Media Preview"
                    className="mt-4 w-64 self-center mb-8 border border-border"
                />
            )}

            {/* Error Feedback */}
            {error && (
                <div className="mt-2 text-sm text-red-500">
                    Error: {error}
                </div>
            )}
            {isUploadError && (
                <p className="profile-settings__updated-error text-brand mt-2">
                    Post media failed to upload. Please try again.
                </p>
            )}
        </div>

    );
};

export default PostForm;