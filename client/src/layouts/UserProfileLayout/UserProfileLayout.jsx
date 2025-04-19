import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";
import SocialPanel from "../../components/SocialPanel/SocialPanel.jsx";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";

const UserProfileLayout = ({ children }) => {
    return (
        <div className=' h-screen flex bg-background'>
            <div className={`border-b border-border`}>
                <Header />
            </div>

            <div className=' pt-17 flex-1 overflow-y-auto '>
                {children}
            </div>

        </div>
);
};

export default UserProfileLayout;