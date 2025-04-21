import {useEffect, useState} from "react";
import {useParams} from "react-router-dom";
import useGetPost from "../../api/hooks/useGetPost.js";
import Post from "../../components/Posts/Post/Post.jsx"
import {refreshComments, refreshPosts} from "../../utils/apiUtils.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate} from "react-router-dom";


const PostObject = () => {

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const {getPost} = useGetPost();
    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, currentHandle, setPosts));
    const navigate = useNavigate();

    const [session_user_id, setSession_user_id] = useState(null);
    const {postId} = useParams();
    const [post, setPost] = useState(null);

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshComments());



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

        const fetchSessionUser = async () => {
            try {
                const response = await getSessionUser();
                setSession_user_id(response.data.user_id);
            } catch (error) {
                console.error("Error fetching session user:", error);
            }
        }

        if (!post) fetchPost();
        if (!session_user_id) fetchSessionUser();
    }, [post, postId, session_user_id]);

    return (
        <div>
            {post ?
                <Post
                    post_id={post.post_id}
                    user_id={post.user_id}
                    session_user_id={session_user_id}
                    handle={post.handle}
                    display_name={post.display_name}
                    date={post.created_at}
                    content={post.content}
                    comments={0}
                    likes={0}
                    onDelete={() => {
                        handleDelete(post.post_id);
                        navigate(`/home`)
                    }}
                    onEdit={() => handleEdit(post.post_id,)}
                    isEditing={editingPostId === post.post_id}
                    editedContent={editedPostContent}
                    onContentChange={(event) =>
                        setEditedPostContent(event.target.value)
                    }
                    onSave={handleSave}
                    refreshComments={refreshComments}
                />

                : "Loading..."}
        </div>
    );
}

export default PostObject;