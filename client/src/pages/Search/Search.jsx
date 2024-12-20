import { useState, useEffect } from "react";
import {NavLink, useLocation} from "react-router-dom";
import "./Search.css";
import "../../api/hooks/useSearch.js"
import useSearch from "../../api/hooks/useSearch.js";

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
                {!loading && results.map((result, index) => (
                    <div key={index} className="search-result__item">
                        {filter === "users" || result.handle ? (
                            <div>
                                <NavLink to={`/profile/${result.handle}`}>{result.display_name} (@{result.handle})</NavLink>
                            </div>
                        ) : (
                            <p>{result.content}</p>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
};

export default Search;
