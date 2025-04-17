import Header from "../../components/Header/Header.jsx";
import Navigation from "../../components/Navigation/Navigation.jsx";

const SettingsLayout = ({ children }) => {
    return (
        <div className='settings-layout flex h-screen pt-17 bg-background'>
            <Header />

            <div className="settings-layout__menu  text-foreground">
                {/* Render Settings Menu */}
                {children[0]}
            </div>
            <main className="settings-layout__main-content flex-1">
                {/* Render Selected Content */}
                {children[1]}
            </main>
        </div>
    );
}

export default SettingsLayout;