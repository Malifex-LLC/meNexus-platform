import Layout_Home from "../Layout/Layout_Home";
import UserPost from "../UserPost/UserPost";
import UserPostEntryField from "../UserPostEntryField/UserPostEntryField";

import axios, {post} from "axios";
import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";

const Home = () => {
    const [posts, setPosts] = useState([]);
    const url = useParams();

    function  getUserPosts() {
        axios.get('/getUserPosts').then((response) => {
            let data = response.data;
            setPosts(data);
            console.log(response.data);
        })
    }

    useEffect(() => {
        getUserPosts();
    }, []); // The empty array tells React to only run the effect once (when the component mounts)

    return (
        //Checks if posts is empty, then maps through the array of UserPosts and renders them all
        <Layout_Home>
            <UserPostEntryField/>
            {posts.length > 0 ? (
                posts.map((post, index) => {
                    return (
                        <UserPost
                            key={index}
                            handle={post.user_handle}
                            username={post.user_name}
                            date={post.post_date}
                            content={post.post_content}
                            comments={post.comment_count}
                            likes={post.likes}
                        />
                    );
                })
            ) : (
                <div>Loading...</div>
            )}

        </Layout_Home>

    )
}

export default Home;