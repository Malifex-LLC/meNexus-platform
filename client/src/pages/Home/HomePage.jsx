import HomeLayout from '../../layouts/HomeLayout.jsx';
import Post from '../../components/Post/Post.jsx';
import PostForm from '../../components/PostForm/PostForm.jsx';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js'
import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";


const HomePage = () => {
    const [posts, setPosts] = useState([]);
    const { handle } = useParams();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();

    useEffect(() => {
        getUserPosts();
    }, []);

    // Handle loading and error states for posts
    if (userPostsLoading) {
        return <div>Loading posts...</div>;
    }

    if (userPostsError) {
        return <div>Error loading posts: {userPostsError.message}</div>;
    }

    return (
        <HomeLayout>
            <PostForm />
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
                        />
                    ))
            ) : (
                <div>Loading...</div>
            )}
        </HomeLayout>
    );
};

export default HomePage;
