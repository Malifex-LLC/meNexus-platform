import './Search.css';
import { useState } from "react";
import { useNavigate } from "react-router-dom";

const Search = () => {
    const [query, setQuery] = useState("");
    const navigate = useNavigate();


    const handleSearch = (e) => {
        e.preventDefault();
        if (query.trim()) {
            navigate(`/search?query=${query}`);
        }
    };

    return (
        <div className="search">
            <label>
                <input
                    className="search__input"
                    type="text"
                    placeholder="Search..."
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key === "Enter") {
                            handleSearch(e);
                        }
                    }}
                />
            </label>
        </div>
    )
};

export default Search;