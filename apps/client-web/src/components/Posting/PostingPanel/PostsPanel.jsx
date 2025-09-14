// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import PostForm from "../PostForm/PostForm.jsx";
import { refreshPosts } from "../../../utils/apiUtils.js";
import Post from "../Post/Post.jsx";
import useEditPost from "../../../api/hooks/useEditPost.js";
import useFetchRemotePosts from "../../../api/hooks/useFetchRemotePosts.js";
import useDeletePost from "../../../api/hooks/useDeletePost.js";
import useGetAllPosts from "../../../api/hooks/useGetAllPosts.js";
import PostBoardsPanel from "../PostBoardsPanel/PostBoardsPanel.jsx";
import useEditRemotePost from "../../../api/hooks/useEditRemotePost.js";
import useDeleteRemotePost from "../../../api/hooks/useDeleteRemotePost.js";

const PostsPanel = ({isLocalSynapse, publicKey, synapsePublicKey, boards, activeBoard, setActiveBoard, posts, setPosts}) => {
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const { getAllPosts } = useGetAllPosts();

    // Hooks for editing and deleting posts

    // Hooks for editing and deleting posts
    const localRefreshPosts = async () => {
        const allPosts = await getAllPosts();
        const filteredPosts = allPosts.filter(post => post.board === activeBoard);
        setPosts(filteredPosts);
    };

    const remoteRefreshPosts = async () => {
        const allPosts = await fetchRemotePosts(synapsePublicKey);
        const filteredPosts = allPosts.filter(post => post.board === activeBoard);
        setPosts(filteredPosts);
    };


    const localEdit = useEditPost(localRefreshPosts);
    const remoteEdit = useEditRemotePost(remoteRefreshPosts, synapsePublicKey);

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = isLocalSynapse ? localEdit : remoteEdit;

    const localDelete = useDeletePost(localRefreshPosts);
    const remoteDelete = useDeleteRemotePost(remoteRefreshPosts, synapsePublicKey);

    const {
        handleDelete
    } = isLocalSynapse ? localDelete : remoteDelete;

    if(!posts) {
        return <div>Loading posts...</div>
    }

    return (
        <div className={'flex flex-col flex-1 overflow-y-auto h-full bg-surface/70 rounded-xl  md:px-8 space-y-8 border border-border'}>
            <div className=" bg-background p-4 rounded-xl mt-4 mx-4  shadow-2xl">
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
                            className={'mx-4 shadow-2xl'}
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
                <div className={`flex flex-col p-4 text-2xl text-center text-foreground`}>No posts to show.</div>
            )}
            <div className={`mb-8`}/>
        </div>
    );
}

export default PostsPanel;