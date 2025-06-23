import Header from "../../components/Header/Header.jsx";

const ExploreLayout = ({children}) => {
    return (
        <div className={'h-screen pt-17 bg-background'}>
            <Header/>
            {children}
        </div>
    );
}

export default ExploreLayout;