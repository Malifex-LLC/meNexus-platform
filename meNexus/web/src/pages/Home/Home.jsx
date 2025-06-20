import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { refreshPosts, refreshComments } from '../../utils/apiUtils.js';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import Post from '../../components/Posts/Post/Post.jsx';
import PostForm from '../../components/Posts/PostForm/PostForm.jsx';
import useGetPosts from "../../api/hooks/useGetPosts.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import useCreatePost from "../../api/hooks/useCreatePost.js";

const Home = () => {
    const { handle } = useParams(); // Extract handle from the URL (if available)
    const [sessionPublicKey, setSessionPublicKey] = useState(null);
    const [currentHandle, setCurrentHandle] = useState(handle || null); // State for the current handle
    const [posts, setPosts] = useState([]); // State for user posts
    const [isHandleSet, setIsHandleSet] = useState(false); // Track if handle is set
    const navigate = useNavigate(); // React Router navigate

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getPosts, loading: postsLoading, error: postsError } = useGetPosts();


    // Hooks for editing and deleting posts
    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getPosts, currentHandle, setPosts));
    const { handleDelete } = useDeletePost(() => refreshPosts(getPosts, currentHandle, setPosts));
    const { createPost, createPostLoading, createPostError } = useCreatePost();

    // Redirect from /home to /home/:handle if no handle is provided
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
                        navigate(`/home/${response.data.handle}`); // Redirect to /home/:handle
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
                setSessionPublicKey(response.data.publicKey);
                setIsHandleSet(true);
            }
        };

        if (!isHandleSet) fetchSessionUser();
    }, [handle]);

    // Fetch posts once the `currentHandle` is determined
    useEffect(() => {
        if (currentHandle && isHandleSet) {
            const fetchPosts = async () => {
                try {
                    console.log("Fetching posts for handle:", currentHandle);
                    const userPostsData = await getPosts();
                    setPosts(userPostsData); // Set the posts
                } catch (error) {
                    console.error("Error fetching posts:", error);
                }
            };
            if (currentHandle && isHandleSet && posts.length === 0) {
                fetchPosts();
            }
        }
    }, [currentHandle, isHandleSet]);

    // Handle loading and error states for posts
    if (postsLoading) {
        return <div>Loading posts...</div>;
    }

    if (postsError) {
        return <div>Error loading posts: {postsError.message}</div>;
    }

    if (sessionUserLoading) {
        return <div>Loading session...</div>;
    }

    if (sessionUserError) {
        return <div>Error loading session user...</div>;
    }

    return currentHandle ? (
        <div className="home__post-container flex flex-col h-full">
            <div className="home__posts flex-1 overflow-y-auto px-8  py-2 space-y-16 ">
                <div className="home__post-form bg-surface p-4 rounded-xl mt-8 ">
                    <PostForm
                        publicKey={sessionPublicKey}
                        createPost={createPost}
                        loading={createPostLoading}
                        error={createPostError}
                        refreshPosts={() => refreshPosts(getPosts, currentHandle, setPosts)}
                    />
                </div>
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) => (
                            <Post
                                key={index}
                                postId={post.post_id}
                                publicKey={post.public_key}
                                sessionPublicKey={sessionPublicKey}
                                handle={post.handle}
                                displayName={post.displayName}
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
    ) : (
        <div>Loading...</div>
    );
};

export default Home;