import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useFetchRemotePosts from "../../api/hooks/useFetchRemotePosts.js"
import PostForm from "../../components/Posts/PostForm/PostForm.jsx";
import {refreshComments, refreshPosts} from "../../utils/apiUtils.js";
import Post from "../../components/Posts/Post/Post.jsx";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import useCreateRemotePost from "../../api/hooks/useCreateRemotePost.js";


const Synapse = () => {
    const { publicKey } = useParams(); // Extract publicKey from the URL (if available)
    const { handle } = useParams(); // Extract handle from the URL (if available)
    const [currentHandle, setCurrentHandle] = useState(handle || null); // State for the current handle
    const [isHandleSet, setIsHandleSet] = useState(false); // Track if handle is set
    const [sessionPublicKey, setSessionPublicKey] = useState(null);
    const [posts, setPosts] = useState([]); // State for synapse posts
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const navigate = useNavigate(); // React Router navigate

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { fetchRemoteSynapseMetadata, loading, error } = useFetchRemoteSynapseMetadata();
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();

    // Hooks for editing and deleting posts
    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(fetchRemotePosts(publicKey), currentHandle, setPosts));
    const { handleDelete } = useDeletePost(() => refreshPosts(fetchRemotePosts(publicKey), currentHandle, setPosts));
    const { createRemotePost, createPostLoading, createPostError } = useCreateRemotePost();

    const handleCreatePost = async ({ content }) => {
        const synapsePublicKey = synapseMetadata.identity.publicKey;
        const publicKey = sessionPublicKey
        const post = {publicKey, content, synapsePublicKey};
        await createRemotePost(post);
        const updatedPosts = await fetchRemotePosts(synapsePublicKey);
        setPosts(updatedPosts)
    }

    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!handle && !isHandleSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();
                    console.log(response);
                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setCurrentHandle(response.data.handle); // Set the handle
                        setIsHandleSet(true); // Mark handle as set
                        setSessionPublicKey(response.data.publicKey);
                    } else {
                        console.error("Invalid session, redirecting to login.");
                        navigate('/login'); // Redirect to login if session is invalid
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                    navigate('/login'); // Redirect to login on error
                }
            } else if (handle) {
                setCurrentHandle(handle); // If handle exists in URL, set it as current
                const response = await getSessionUser();
                setSessionPublicKey(response.data.user_id);
                setIsHandleSet(true);
            }
        };
        if (!isHandleSet) fetchSessionUser();
    }, [getSessionUser, handle, isHandleSet, navigate]);

    useEffect(() => {
        const fetchSynapseData = async () => {
            try {
                const synapsePostsData = await fetchRemotePosts(publicKey);
                setPosts(synapsePostsData);
                const synapseMetadataResponse = await fetchRemoteSynapseMetadata(publicKey);
                setSynapseMetadata(synapseMetadataResponse);
            } catch (error) {
                console.error("Error fetching Synapse data", error);
            }
        };
        fetchSynapseData();
    },[publicKey]);

    if (synapsePostsLoading) {
        return <div>Loading Synapse Posts...</div>
    }

    if (synapsePostsError) {
        return <div>Error Loading Synapse Posts: {synapsePostsError.message}</div>
    }

    return(
        <div className="bg-background ">
            {synapseMetadata ? (
                <header className="p-4 text-foreground">
                    <h1 className="text-7xl text-brand font-bold">{synapseMetadata.metadata.name}</h1>
                    <p className="text-3xl text-foreground">
                        {synapseMetadata.metadata.description}
                    </p>
                    <code className="text-xs block mt-2">
                        publicKey: {synapseMetadata.identity.publicKey}
                    </code>
                </header>
            ) : (
                <div className="p-4">Loading synapse info…</div>
            )}
            <div className="home__posts flex-1 overflow-y-auto px-8  py-2 space-y-16 ">
                <div className="home__post-form bg-surface p-4 rounded-xl mt-8 ">
                    <PostForm
                        publicKey={publicKey}
                        createPost={handleCreatePost}
                        loading={createPostLoading}
                        error={createPostError}
                        refreshPosts={() => refreshPosts(fetchRemotePosts(publicKey), publicKey, setPosts)}
                    />
                </div>
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) => (
                            <Post
                                key={index}
                                post_id={post.post_id}
                                user_id={post.user_id}
                                session_user_id={sessionPublicKey}
                                handle={post.handle}
                                display_name={post.display_name}
                                date={post.created_at}
                                content={post.content}
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
                                refreshComments={refreshComments}
                            />
                        ))
                ) : (
                    <div>No posts to show.</div>
                )}
            </div>
        </div>
    );
};

export default Synapse;