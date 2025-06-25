// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate, useParams} from "react-router-dom";
import Post from "../../pages/Post/PostObject.jsx";
import useGetPost from "../../api/hooks/useGetPost.js";

const PostLayout = ({children}) => {
    const navigate = useNavigate(); // React Router navigate
    const {postId} = useParams();
    console.log("postId from params:", postId);

    const [user, setUser] = useState(null)
    const [post, setPost] = useState(null)
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getPost } = useGetPost();

    useEffect(() => {
        const fetchUser = async () => {
            try {
                console.log("Fetching current user session...");
                const sessionUserResponse = await getSessionUser();
                const userResponse = await getUser(sessionUserResponse.data.publicKey);
                setUser(userResponse.data);
            } catch (error) {
                console.error("Error fetching current session user:", error);
                navigate('/login');
            }
        }
        fetchUser();
    }, [])

    useEffect(() => {
        const fetchPost = async () => {
            try {
                console.log("Fetching post with id: ", postId);
                const fetchedPost = await getPost(postId);
                console.log("fetchedPost: ", fetchedPost);
                setPost(fetchedPost);
            } catch (error) {
                console.error("Error getting post:", error);
            }
        }
        fetchPost();
    }, [postId]);


    if (!user) {
        return (
            <div className={'h-screen bg-background text-foreground text-center p-4'}>Loading post...</div>
        )
    }

    return (
        <div className='post-layout flex flex-col w-screen h-screen items-center justify-center  bg-background'>
            <div className='sticky top-0 z-50 border-b border-border'>
                <Header
                    user={user}
                />
            </div>
            <main className='post-layout__main-content  p-4 lg:mx-32 xl:mx-64'>
                <Post
                    isLocalSynapse={true}
                synapsePublicKey={"03f9fc50468c1f3fbd3db448d66cc7ae07d0e379522474504c1f6d812804366d3e"}
                postId={postId}
                publicKey={post.publicKey}
                sessionPublicKey={user.publicKey}
                date={post.date}
                content={post.content}

                />
                {children}
            </main>
        </div>
    );
}

export default PostLayout;