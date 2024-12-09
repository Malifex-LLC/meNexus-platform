import './Home.css'
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {refreshPosts} from '../../utils/apiUtils.js'
import Post from '../../components/Post/Post.jsx';
import PostForm from '../../components/PostForm/PostForm.jsx';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js'
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";


const Home = () => {
    const [posts, setPosts] = useState([]);
    const { handle } = useParams();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getUserPosts, handle, setPosts));

    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, handle, setPosts));

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [userPostsData] = await Promise.all([
                    getUserPosts(handle),
                ]);
                setPosts(userPostsData);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        };
        fetchData();
    }, []);

    // Handle loading and error states for posts
    if (userPostsLoading) {
        return <div>Loading posts...</div>;
    }

    if (userPostsError) {
        return <div>Error loading posts: {userPostsError.message}</div>;
    }

    return (
        <div className="home__post-container">
            <div className="home__post-form">
                <PostForm handle={handle} refreshPosts={() => refreshPosts(getUserPosts, handle, setPosts)} />
            </div>
            <div className="home__posts">
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) => (
                            <Post
                                key={index}
                                handle={post.handle}
                                username={post.username}
                                date={post.created_at}
                                content={post.content}
                                comments={post.comment_count}
                                likes={post.likes}
                                onDelete={() => handleDelete(post.post_id)}
                                onEdit={() => handleEdit(post.post_id, posts)}
                                isEditing={editingPostId === post.post_id}
                                editedContent={editedPostContent}
                                onContentChange={(event) =>
                                    setEditedPostContent(event.target.value)
                                }
                                onSave={handleSave}
                            />
                        ))
                ) : (
                    <div>Loading...</div>
                )}
            </div>
        </div>
    );
};

export default Home;
