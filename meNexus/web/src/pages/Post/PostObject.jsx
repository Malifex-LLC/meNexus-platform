import {useEffect, useState} from "react";
import {useParams} from "react-router-dom";
import useGetPost from "../../api/hooks/useGetPost.js";
import Post from "../../components/Posting/Post/Post.jsx"
import {refreshComments, refreshPosts} from "../../utils/apiUtils.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import {useNavigate} from "react-router-dom";
import useGetUserPosts from "../../api/hooks/useGetUserPosts.js";


const PostObject = ({sessionUser}) => {
    const [posts, setPosts ] = useState([]);
    const {getUserPosts} = useGetUserPosts();

    const { getPost } = useGetPost();
    const { handleDelete } = useDeletePost(() => refreshPosts(getUserPosts, sessionUser.publicKey, setPosts));
    const navigate = useNavigate();

    const {postId} = useParams();
    console.log("postId from params:", postId);

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
        fetchPost();
    }, [postId]);

    if (!sessionUser) {
        return (
            <div className={'h-screen bg-background text-foreground text-center p-4'}>Loading a post...</div>
        )
    }

    return (
        <div>
            {post ?
                <Post
                    postId={post.post_id}
                    publicKey={post.public_key}
                    sessionPublicKey={sessionUser.publicKey}
                    handle={post.handle}
                    displayName={post.displayName}
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