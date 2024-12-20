import "./UserProfile.css";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { refreshPosts } from '../../utils/apiUtils.js';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import useGetProfile from '../../api/hooks/useGetProfile.js';
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js';
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import Post from "../../components/Posts/Post/Post.jsx";
import PostForm from "../../components/Posts/PostForm/PostForm.jsx";

const UserProfile = () => {
    const { handle } = useParams();
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [profile, setProfile] = useState({});
    const [posts, setPosts] = useState([]);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const [isFollowing, setIsFollowing] = useState(false);
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", profile.user_id);
        try {
            await followUser(profile.user_id);
            setIsFollowing(true);
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", profile.user_id);
        try {
            await unfollowUser(profile.user_id);
            setIsFollowing(false);
        } catch (err) {
            console.error('Error unfollowing user:', err);
        }
    };

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
                    console.log("Profile Data after getProfile() fetching is:", profileData);
                    setProfile(profileData);
                    setPosts(userPostsData);

                    const isCurrentlyFollowing = await followCheck(profileData.user_id);
                    console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                    setIsFollowing(isCurrentlyFollowing);
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
                    <img src={`http://localhost:3001${profile.profile_picture}`} alt="Profile Picture"/>
                </div>
                <button
                    className="user-profile__follow-button"
                    onClick={isFollowing ? handleUnfollow : handleFollow}                >
                    {isFollowing ? "Unfollow" : "Follow"}
                </button>
                <div className="user-profile__info">
                    <h2 className="user-profile__name">{profile.profile_name}</h2>
                    <p className="user-profile__bio">{profile.profile_bio}</p>
                    <p className="user-profile__location">{profile.profile_location}</p>
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
                                    user_id={post.user_id}
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