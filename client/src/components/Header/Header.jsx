import './Header.css';
import '../Search/Search.jsx'
import Search from "../Search/Search.jsx";

const Header = () => {
    return (
        <div className="header__container">
            <header
                className='header__main-content'
                role='banner'
                aria-label='Main Header'
            >
                <h1>meNexus</h1>
            </header>
            <div className="header__search">
                <Search />
            </div>
        </div>
    )
};

export default Header;