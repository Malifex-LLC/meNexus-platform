import PostForm from "../PostForm/PostForm.jsx";
import { refreshPosts } from "../../../utils/apiUtils.js";
import Post from "../Post/Post.jsx";
import useEditPost from "../../../api/hooks/useEditPost.js";
import useFetchRemotePosts from "../../../api/hooks/useFetchRemotePosts.js";
import useDeletePost from "../../../api/hooks/useDeletePost.js";
import useCreateRemotePost from "../../../api/hooks/useCreateRemotePost.js";
import useCreatePost from "../../../api/hooks/useCreatePost.js";
import useGetAllPosts from "../../../api/hooks/useGetAllPosts.js";

const PostsPanel = ({isLocalSynapse, publicKey, synapsePublicKey, posts, setPosts}) => {
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const { getAllPosts } = useGetAllPosts();
    // Hooks for editing and deleting posts
    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));

    const { handleDelete } = useDeletePost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));


    return (
        <div className="home__posts h-full flex-1 overflow-y-auto px-8  py-2 space-y-16 ">
            <div className="home__post-form bg-surface p-4 rounded-xl mt-8 ">
                <PostForm
                    isLocalSynapse={isLocalSynapse}
                    publicKey={publicKey}
                    synapsePublicKey={synapsePublicKey}
                    refreshPosts={() => (
                        isLocalSynapse
                            ? refreshPosts(() => getAllPosts(), setPosts)
                            : refreshPosts(() => fetchRemotePosts(synapsePublicKey), setPosts)
                    )}
                />
            </div>
            {posts.length > 0 ? (
                posts
                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                    .map((post, index) => (
                        <Post
                            key={index}
                            isLocalSynapse={isLocalSynapse}
                            synapsePublicKey={synapsePublicKey}
                            postId={post.post_id}
                            publicKey={post.public_key}
                            session_user_id={publicKey}
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
    );
}

export default PostsPanel;