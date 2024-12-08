import "./UserProfile.css";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import useGetProfile from '../../api/hooks/useGetProfile.js';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js'
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import Post from "../../components/Post/Post.jsx";
import PostForm from "../../components/PostForm/PostForm.jsx";


const UserProfile = () => {
    const { handle } = useParams();
    const [profile, setProfile] = useState({});
    const [posts, setPosts] = useState([]);

    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();

    const refreshPosts = async () => {
        try {
            const userPostsData = await getUserPosts(handle);
            setPosts(userPostsData);
        } catch (error) {
            console.log("Error refreshing posts:", error);
        }
    };

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(refreshPosts);

    const { handleDelete } = useDeletePost(refreshPosts);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [profileData, userPostsData] = await Promise.all([
                    getProfile(handle),
                    getUserPosts(handle),
                ]);
                setProfile(profileData);
                setPosts(userPostsData);
            } catch (error) {
                console.error("Error fetching data:", error);
            }
        };

        fetchData();
    }, []);

    // Handle loading and error states for profile
    if (profileLoading) {
        return <div>Loading profile...</div>;
    }

    if (profileError) {
        return <div>Error loading profile: {profileError.message}</div>;
    }

    // Handle loading and error states for posts
    if (userPostsLoading) {
        return <div>Loading posts...</div>;
    }

    if (userPostsError) {
        return <div>Error loading posts: {userPostsError.message}</div>;
    }

    return (
        <div className="profile-container">
            <div className="profile-header">
                <div className="profile-picture">
                    <img src={profile.profile_picture} alt="Profile" />
                </div>
                <div className="profile-info">
                    <h2 className="profile-name">{profile.name}</h2>
                    <p className="profile-bio">{profile.bio}</p>
                    <p className="profile-location">{profile.location}</p>
                </div>
            </div>
            <div className="profile-posts">
                <PostForm handle={handle} refreshPosts={refreshPosts} />
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

export default UserProfile;
