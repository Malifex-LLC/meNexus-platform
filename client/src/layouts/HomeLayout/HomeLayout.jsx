import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import SocialPanel from "../../components/SocialPanel/SocialPanel.jsx";

const HomeLayout = ({ children }) => {
    return (
        <div className='home-layout h-screen flex bg-background'>
            <div className={`border-b border-border`}>
                <Header />
            </div>
            <div className={` flex flex-col pt-17  border-r border-border `}>
                <SocialPanel/>
            </div>
            <div className='home-layout__main-content pt-17 px-4 flex-1 overflow-hidden'>
                {children}
            </div>
            <div className={`flex flex-col pt-17  border-l border-border`}>
                <ActivityFeed/>
            </div>
        </div>
    );
}

export default HomeLayout;