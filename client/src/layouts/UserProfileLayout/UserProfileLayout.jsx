import './UserProfileLayout.css'
import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";

const UserProfileLayout = ({ children }) => {
    return (
        <div className='user-profile-layout'>
            <Header />
            <Navigation />
            <main className='user-profile-layout__main-content'>
                <div className="user-profile-layout__container">
                    <div className="user-profile-layout__user-profile">
                        {/* User Profile Content */}
                        {children}
                    </div>
                </div>
            </main>
        </div>
);
};

export default UserProfileLayout;