import { useState } from "react";
import SettingsLayout from "../../layouts/SettingsLayout/SettingsLayout.jsx";
import SettingsMenu from "../../components/Settings/SettingsMenu/SettingsMenu.jsx";
import AccountSettings from "../../components/Settings/AccountSettings/AccountSettings.jsx";
import ProfileSettings from "../../components/Settings/ProfileSettings/ProfileSettings.jsx";
import DisplaySettings from "../../components/Settings/DisplaySettings/DisplaySettings.jsx";

const SettingsPage = () => {
    const [selectedMenu, setSelectedMenu] = useState('Profile');

    // Render content based on selection
    const renderContent = () => {
        switch (selectedMenu) {
            case 'Account':
                return (
                    <AccountSettings />
                );
            case 'Profile':
                return (
                    <ProfileSettings />
                );
            case 'Display':
                return (
                    <DisplaySettings />
                );
            default:
                return <div>Select an option from the menu.</div>;
        }
    };

    return (
        <SettingsLayout>
            <SettingsMenu
                onSelectMenu={setSelectedMenu}
                selectedMenu={selectedMenu}
            />
            {renderContent()}
        </SettingsLayout>
    );
};

export default SettingsPage;