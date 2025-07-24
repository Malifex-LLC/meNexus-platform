// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import PostForm from "../PostForm/PostForm.jsx";
import { refreshPosts } from "../../../utils/apiUtils.js";
import Post from "../Post/Post.jsx";
import useEditPost from "../../../api/hooks/useEditPost.js";
import useFetchRemotePosts from "../../../api/hooks/useFetchRemotePosts.js";
import useDeletePost from "../../../api/hooks/useDeletePost.js";
import useCreateRemotePost from "../../../api/hooks/useCreateRemotePost.js";
import useCreatePost from "../../../api/hooks/useCreatePost.js";
import useGetAllPosts from "../../../api/hooks/useGetAllPosts.js";
import PostBoardsPanel from "../PostBoardsPanel/PostBoardsPanel.jsx";
import {useState} from "react";

const PostsPanel = ({isLocalSynapse, publicKey, synapsePublicKey, boards, activeBoard, setActiveBoard, posts, setPosts}) => {
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const { getAllPosts } = useGetAllPosts();

    // Hooks for editing and deleting posts
    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));

    const { handleDelete } = useDeletePost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));


    return (
        <div className="flex flex-row h-full rounded-xl">
            <div className={'bg-background p-4 mr-2  rounded-xl  w-1/5 text-2xl text-foreground border border-border shadow-2xl'}
            >
                <PostBoardsPanel
                    boards={boards}
                    activeBoard={activeBoard}
                    setActiveBoard={setActiveBoard}
                />
            </div>
            <div className={'home__posts bg-background rounded-xl h-full flex-1 overflow-y-auto px-8 space-y-8 border border-border'}>
                <div className="home__post-form bg-surface p-4 rounded-xl mt-4  shadow-2xl">
                    <PostForm
                        isLocalSynapse={isLocalSynapse}
                        publicKey={publicKey}
                        synapsePublicKey={synapsePublicKey}
                        activeBoard={activeBoard}
                        refreshPosts={() => (
                            isLocalSynapse
                                ? refreshPosts(() => getAllPosts(), setPosts)
                                : refreshPosts(() => fetchRemotePosts(synapsePublicKey), setPosts)
                        )}
                    />
                </div>
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) => (
                            <div
                                key={index}
                                className={'shadow-2xl mx-32'}
                            >
                                <Post
                                    key={index}
                                    isLocalSynapse={isLocalSynapse}
                                    synapsePublicKey={synapsePublicKey}
                                    synapseUrl={post.synapseUrl}
                                    postId={post.post_id}
                                    publicKey={post.public_key}
                                    sessionPublicKey={publicKey}
                                    date={post.created_at}
                                    content={post.content}
                                    mediaUrl={post.media_url}
                                    comments={0}
                                    likes={0}
                                    onDelete={() => handleDelete(post.post_id)}
                                    onEdit={() => handleEdit(post.post_id, posts)}
                                    isEditing={editingPostId === post.post_id}
                                    editedContent={editedPostContent}
                                    onContentChange={(event) =>
                                        setEditedPostContent(event.target.value)
                                    }
                                    onSave={handleSave}
                                />
                            </div>
                        ))
                ) : (
                    <div>No posts to show.</div>
                )}
            </div>
        </div>
    );
}

export default PostsPanel;