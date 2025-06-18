import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import {refreshComments, refreshPosts} from '../../utils/apiUtils.js';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import useGetUserPosts from '../../api/hooks/useGetUserPosts.js';
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import useCreateNotification from "../../api/hooks/useCreateNotification.js";
import Post from "../../components/Posts/Post/Post.jsx";
import PostForm from "../../components/Posts/PostForm/PostForm.jsx";
import { IoLocationSharp } from "react-icons/io5";
import useGetUserByHandle from "../../api/hooks/useGetUserByHandle.js";


const UserProfile = () => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUserByHandle, loading: userLoading, error: userError } = useGetUserByHandle();
    const { getUserPosts, loading: userPostsLoading, error: userPostsError } = useGetUserPosts();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { createNotification } = useCreateNotification();
    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getUserPosts, currentHandle, setPosts));

    const { handle } = useParams();
    const [user, setUser] = useState(null)
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [sessionPublicKey, setSessionPublicKey] = useState(null);
    const [sessionUserHandle, setSessionUserHandle] = useState(null);
    const [posts, setPosts] = useState([]);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const [isFollowing, setIsFollowing] = useState(false);
    const navigate = useNavigate();

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", user.publicKey);
        const notification = {
            public_key: user.publicKey,
            actor_public_key: sessionPublicKey,
            resource_type: "FOLLOW",
            resource_id: sessionPublicKey,
            action: "FOLLOW",
        }
        try {
            await followUser(user.publicKey);
            setIsFollowing(true);
            await createNotification(notification);
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", user.publicKey);
        try {
            await unfollowUser(user.publicKey);
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
                    const userData = await getUserByHandle(handle);
                    setUser(userData);
                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setCurrentHandle(response.data.handle);
                        setIsHandleSet(true);
                        setSessionPublicKey(response.data.publicKey);
                        setSessionUserHandle(response.data.handle);
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
                const response = await getSessionUser();
                const userData = await getUserByHandle(handle);
                setSessionPublicKey(response.data.publicKey);
                setSessionUserHandle(response.data.handle);
                setIsHandleSet(true);
                setUser(userData);
            }
        };
        fetchSessionUser();
    }, [handle]);

    // Fetch profile and posts once the current handle is determined
    useEffect(() => {
        if (currentHandle && isHandleSet && user) {
            const fetchData = async () => {
                try {
                    const userPostsData = await getUserPosts(user.publicKey);
                    setPosts(userPostsData);
                    const isCurrentlyFollowing = await followCheck(user.publicKey);
                    console.log('userProfile calling followCheck for followedPublicKey: ', user.publicKey);
                    console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                    setIsFollowing(isCurrentlyFollowing);
                } catch (error) {
                    console.error("Error fetching data:", error);
                }
            };
            fetchData();
        }
    }, [currentHandle, isHandleSet, user]);

    // Handle loading and error states for posts
    if (userPostsLoading) {
        return <div>Loading posts...</div>;
    }

    if (userPostsError) {
        return <div>Error loading posts: {userPostsError.message}</div>;
    }

    if (!user) {
        return <div>Loading user...</div>
    }

    const isOwner = user?.publicKey === sessionPublicKey;
    return user ? (
        <div className="user-profile__container flex flex-col lg:grid grid-cols-12 p-16  ">
            <div className="user-profile__data flex flex-col lg:col-span-3 p-4 items-center
            bg-surface text-foreground rounded-2xl">
                <div className="user-profile__picture w-32 items-center">
                    <img src={user.profilePicture}
                         alt="Profile Picture" />
                </div>
                {!isOwner && (
                    <button
                        className="user-profile__follow-button rounded-xl bg-brand"
                        onClick={isFollowing ? handleUnfollow : handleFollow}>
                        {isFollowing ? "Unfollow" : "Follow"}
                    </button>
                )}
                <div className="user-profile__info p-4 ">
                    <h2 className="user-profile__name text-4xl">{user.profileName}</h2>
                    <h3 className="user-profile__handle text-4xl">{handle}</h3>
                    <p className="user-profile__bio text-2xl">{user.bio}</p>
                    <div className={`text-2xl flex items-center gap-2`} >
                        <IoLocationSharp />
                        <p className="user-profile__location ">{user.location}</p>

                    </div>

                </div>
            </div>
            <div className="user-profile__post-container flex flex-col lg:col-span-9 h-full">
                <div className="user-profile__posts flex-1 overflow-y-auto px-4 py-2 space-y-16  ">
                    {isOwner && (
                        <div className="user-profile__post-form bg-surface p-4 rounded-xl mt-8 ">
                            <PostForm
                                handle={currentHandle}
                                refreshPosts={() => refreshPosts(getUserPosts, currentHandle, setPosts)} />
                        </div>
                    )}
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
        </div>
    ) : (
        <div>Loading...</div>
    );
};

export default UserProfile;