import './Search.css';

const Search = () => {
    return (
        <div className="search">
            <label>
                <input
                    className="search__input"
                    type="text"
                    placeholder="Search..."
                />
            </label>
        </div>
    )
};

export default Search;