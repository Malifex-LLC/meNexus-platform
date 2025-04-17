import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";

const SearchLayout = ({children}) => {
    return (
        <div className="search-layout flex  pt-17 bg-background text-foreground">
            <Header />
            <main className="search-layout__main-content">
                {children}
            </main>
        </div>
    )
};

export default SearchLayout;