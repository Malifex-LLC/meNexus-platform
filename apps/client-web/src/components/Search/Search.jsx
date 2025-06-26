// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

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
        <div className="search border w-32 md:w-full  border-border lg:rounded-xl">
            <label>
                <input
                    className="search__input w-32 md:w-full lg:px-4 bg-surface lg:rounded-xl"
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