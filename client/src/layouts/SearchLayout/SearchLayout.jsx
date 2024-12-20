import './SearchLayout.css'
import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";

const SearchLayout = ({children}) => {
    return (
        <div className="search-layout">
            <Header />
            <Navigation />
            <main className="search-layout__main-content">
                {children}
            </main>
        </div>
    )
};

export default SearchLayout;