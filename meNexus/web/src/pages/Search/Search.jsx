import { useState, useEffect } from "react";
import {useLocation, useNavigate} from "react-router-dom";
import "./Search.css";
import "../../api/hooks/useSearch.js"
import useSearch from "../../api/hooks/useSearch.js";
import ProfileCard from "../../components/ProfileCard/ProfileCard.jsx";
import Post from "../../components/Posts/Post/Post.jsx"
import {refreshComments, refreshPosts} from "../../utils/apiUtils.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetPosts from "../../api/hooks/useGetPosts.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";

const Search = () => {
    const location = useLocation();
    const queryParams = new URLSearchParams(location.search);
    const query = queryParams.get("query");
    const [results, setResults] = useState([]);
    const [filter, setFilter] = useState("all"); // Options: all, users, posts
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const {search} = useSearch();
    const navigate = useNavigate();
    const [sessionPublicKey, setsessionPublicKey] = useState(null);


    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getPosts, loading: postsLoading, error: postsError } = useGetPosts();

    // Hooks for editing and deleting posts
    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() => refreshPosts(getPosts, currentHandle, setPosts));
    const { handleDelete } = useDeletePost(() => refreshPosts(getPosts, currentHandle, setPosts));

    useEffect(() => {
        const fetchResults = async () => {
            setLoading(true);
            setError(null);
            try {
                const response = await search({ query, type: filter === "all" ? null : filter });
                setResults(response.results);
            } catch (err) {
                setError("Failed to fetch search results.");
                throw err;
            } finally {
                setLoading(false);
            }
        };

        fetchResults();
    }, [query, filter]);

    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                const response = await getSessionUser();

                if (response.status === 200 && response.data.publicKey) {
                    setsessionPublicKey(response.data.publicKey);
                } else {
                    console.error("Invalid session, redirecting to login.");
                    navigate ("/login");
                }
            } catch (error) {
                console.error(error);
            }
        }
        fetchSessionUser();
    }, [])

    const userResults = results.filter((result) => result.type === "user");
    const postResults = results.filter((result) => result.type === "post");

    return (
        <div className="search-results h-screen overflow-hidden flex flex-col ">
            <h1>Search Results for "{query}"</h1>

            <div className="search-results__content flex-1 overflow-hidden flex gap-16 px-8 ">
                {loading && <p>Loading...</p>}
                {error && <p className="error">{error}</p>}
                {!loading && !error && results.length === 0 && <p>No results found.</p>}
                {!loading && !error && (
                    <div className={'flex gap-16'}>
                        {/* Render User Results */}
                        {userResults.length > 0 && (
                            <div className="search-results__users-container flex-1 overflow-y-auto p-4 space-y-8 ">
                                <h1 className={`font-semibold`}>Users Found:</h1>
                                {userResults.map((user, index) => (
                                    <ProfileCard
                                        key={index}
                                        publicKey={user.publicKey}
                                        handle={user.handle}
                                    />
                                ))}
                            </div>
                        )}
                        {/* Render Post Results */}
                        {postResults.length > 0 && (
                            <div className="search-results__posts-container flex-1 overflow-y-auto p-4 space-y-4">
                                <h1 className={`font-semibold`}>Posts Found:</h1>
                                {postResults
                                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                                    .map((post, index) => (
                                    <Post
                                        key={index}
                                        post_id={post.post_id}
                                        publicKey={post.public_key}
                                        sessionPublicKey={sessionPublicKey}
                                        handle={post.handle}
                                        displayName={post.displayName}
                                        date={post.created_at}
                                        content={post.content}
                                        comments={0}
                                        likes={0}
                                        onDelete={() => handleDelete(post.post_id)}
                                        onEdit={() => handleEdit(post.post_id, postResults)}
                                        isEditing={editingPostId === post.post_id}
                                        editedContent={editedPostContent}
                                        onContentChange={(event) =>
                                            setEditedPostContent(event.target.value)
                                        }
                                        onSave={handleSave}
                                        refreshComments={refreshComments}
                                    />
                                ))}
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};

export default Search;