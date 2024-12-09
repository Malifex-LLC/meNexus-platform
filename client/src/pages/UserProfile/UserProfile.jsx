import "./UserProfile.css";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {refreshPosts} from '../../utils/apiUtils.js'
import useGetProfile from '../../api/hooks/useGetProfile.js';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js'
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import Post from "../../components/Post/Post.jsx";
import PostForm from "../../components/PostForm/PostForm.jsx";
import profilePic from '../../assets/profile_pic.jpg'


const UserProfile = () => {
    const { handle } = useParams();
    const [profile, setProfile] = useState({});
    const [posts, setPosts] = useState([]);

    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
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
                const [profileData, userPostsData] = await Promise.all([
                    getProfile(handle),
                    getUserPosts(handle),
                ]);
                console.log("Profile Data:", profileData);
                setProfile(profileData[0]); //profileData is an array with one object so we access it with [0]
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
        <div className="user-profile__container">
            <div className="user-profile__data">
                <div className="user-profile__picture">
                    <img src={profilePic} alt="Profile Picture" />
                </div>
                <div className="user-profile__info">
                    <h2 className="user-profile__name">{profile.name}</h2>
                    <p className="user-profile__bio">{profile.bio}</p>
                    <p className="user-profile__location">{profile.location}</p>
                </div>
            </div>
            <div className="user-profile__post-container">
                <div className="user-profile__post-form">
                    <PostForm handle={handle} refreshPosts={() => refreshPosts(getUserPosts, handle, setPosts)} />
                </div>
                <div className="user-profile__posts">
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
        </div>
    );
};

export default UserProfile;
