import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import "./Search.css";
import "../../api/hooks/useSearch.js"
import useSearch from "../../api/hooks/useSearch.js";
import ProfileCard from "../../components/ProfileCard/ProfileCard.jsx";
import Post from "../../components/Posts/Post/Post.jsx"

const Search = () => {
    const location = useLocation();
    const queryParams = new URLSearchParams(location.search);
    const query = queryParams.get("query");
    const [results, setResults] = useState([]);
    const [filter, setFilter] = useState("all"); // Options: all, users, posts
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const {search} = useSearch();

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
                                {postResults.map((post, index) => (
                                    <Post
                                        key={index}
                                        user_id={post.user_id}
                                        display_name={post.display_name}
                                        handle={post.handle}
                                        content={post.content}
                                        date={post.created_at}
                                        likes={post.likes || 0} // Assuming likes is a field
                                        comments={post.comments || 0} // Assuming comments is a field
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