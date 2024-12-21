import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
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

    const userResults = results.filter((result) => result.type === "user");
    const postResults = results.filter((result) => result.type === "post");

    return (
        <div className="search-results">
            <h1>Search Results for "{query}"</h1>
            <div className="search-results__filters">
                <button onClick={() => setFilter("all")} className={filter === "all" ? "active" : ""}>
                    All
                </button>
                <button onClick={() => setFilter("users")} className={filter === "users" ? "active" : ""}>
                    Users
                </button>
                <button onClick={() => setFilter("posts")} className={filter === "posts" ? "active" : ""}>
                    Posts
                </button>
            </div>
            <div className="search-results__content">
                {loading && <p>Loading...</p>}
                {error && <p className="error">{error}</p>}
                {!loading && !error && results.length === 0 && <p>No results found.</p>}
                {!loading && !error && (
                    <>
                        {/* Render User Results */}
                        {userResults.length > 0 && (
                            <div className="search-results__users-container">
                                <h1>Users Found:</h1>
                                {userResults.map((user, index) => (
                                    <ProfileCard key={index} handle={user.handle} />
                                ))}
                            </div>
                        )}
                        {/* Render Post Results */}
                        {postResults.length > 0 && (
                            <div className="search-results__posts-container">
                                <h1>Posts Found:</h1>
                                {postResults
                                    .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                                    .map((post, index) => (
                                    <Post
                                        key={index}
                                        post_id={post.post_id}
                                        user_id={post.user_id}
                                        handle={post.handle}
                                        display_name={post.display_name}
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
                    </>
                )}
            </div>
        </div>
    );
};

export default Search;