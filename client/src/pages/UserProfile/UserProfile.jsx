import "./UserProfile.css";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { refreshPosts } from '../../utils/apiUtils.js';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import useGetProfile from '../../api/hooks/useGetProfile.js';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js';
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import Post from "../../components/Posts/Post/Post.jsx";
import PostForm from "../../components/Posts/PostForm/PostForm.jsx";
import profilePic from '../../assets/profile_pic.jpg';

const UserProfile = () => {
    const { handle } = useParams();
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [profile, setProfile] = useState({});
    const [posts, setPosts] = useState([]);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    // Redirect from /profile to /profile/:handle if no handle is provided
    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!handle && !isHandleSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();

                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setCurrentHandle(response.data.handle);
                        setIsHandleSet(true);
                        navigate(`/profile/${response.data.handle}`);
                    } else {
                        console.error("Invalid session, redirecting to login.");
                        navigate('/login');
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                    navigate('/login');
                }
            } else if (handle) {
                setCurrentHandle(handle);
                setIsHandleSet(true);
            }
        };

        fetchSessionUser();
    }, [handle, navigate, isHandleSet]);

    // Fetch profile and posts once the current handle is determined
    useEffect(() => {
        if (currentHandle && isHandleSet) {
            const fetchData = async () => {
                try {
                    console.log("Fetching profile and posts for handle:", currentHandle);
                    const [profileData, userPostsData] = await Promise.all([
                        getProfile(currentHandle),
                        getUserPosts(currentHandle),
                    ]);
                    setProfile(profileData[0]);
                    setPosts(userPostsData);
                } catch (error) {
                    console.error("Error fetching data:", error);
                }
            };

            fetchData();
        }
    }, [currentHandle, isHandleSet]);

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

    return currentHandle ? (
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
                    <PostForm
                        handle={currentHandle}
                        refreshPosts={() => refreshPosts(getUserPosts, currentHandle, setPosts)} />
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
                        <div>No posts to show.</div>
                    )}
                </div>
            </div>
        </div>
    ) : (
        <div>Loading...</div>
    );
};

export default UserProfile;