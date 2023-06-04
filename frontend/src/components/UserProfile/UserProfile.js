import React, { useEffect, useState } from "react";
import axios from "axios";
import { useParams } from "react-router-dom";
import Post from "../Post/Post";
import PostForm from "../PostForm/PostForm";
import "./UserProfile.css";

const UserProfile = () => {
    const [profile, setProfile] = useState({});
    const [posts, setPosts] = useState([]);

    const { handle } = useParams();

    function getProfile() {
        axios.get(`/getProfile/${handle}`).then((response) => {
            let data = response.data;
            if (Array.isArray(data) && data.length > 0) {
                setProfile(data[0]);
            }
            console.log(response.data);
        });
    }

    function getUserPosts() {
        axios.get(`/getUserPosts/${handle}`).then((response) => {
            let data = response.data;
            setPosts(data);
            console.log(response.data);
        });
    }

    function handleDelete(post_id) {
        console.log("Deleting post:", post_id);
        axios.delete(`/deletePost/${post_id}`).then((response) => {
            console.log(response.data);
            getUserPosts(); // Refresh the posts after deleting the post
        });
    }

    useEffect(() => {
        getProfile();
        getUserPosts();
    }, []);

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
                <PostForm handle={handle} />
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
                                onDelete={() => handleDelete(post.post_id)} // Use post.post_id instead of post.id
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
