import './SettingsMenu.css';

const SettingsMenu = ({ onSelectMenu }) => {
    return (
        <ul className="settings-menu__list">
            <li onClick={() => onSelectMenu('Account')} className="settings-menu__item">
                Account
            </li>
            <li onClick={() => onSelectMenu('Profile')} className="settings-menu__item">
                Profile
            </li>
            <li onClick={() => onSelectMenu('Display')} className="settings-menu__item">
                Display
            </li>
        </ul>
    );
};

export default SettingsMenu;