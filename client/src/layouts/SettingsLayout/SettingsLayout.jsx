import './SettingsLayout.css';
import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";

const SettingsLayout = ({ children }) => {
    return (
        <div className='settings-layout'>
            <Header />
            <Navigation />
            <div className="settings-layout__menu">
                {/* Render Settings Menu */}
                {children[0]}
            </div>
            <main className="settings-layout__main-content">
                {/* Render Selected Content */}
                {children[1]}
            </main>
        </div>
    );
}

export default SettingsLayout;