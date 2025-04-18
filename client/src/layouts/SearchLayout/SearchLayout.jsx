import Header from "../../components/Header/Header.jsx";

const SearchLayout = ({children}) => {
    return (
        <div className="search-layout h-full pt-17   bg-background text-foreground">
            <Header />
            <main className="search-layout__main-content ">
                {children}
            </main>
        </div>
    )
};

export default SearchLayout;