import './Header.css';
import '../Search/Search.jsx'
import { useState } from "react";
import NotificationsTray from '../Notifications/NotificationsTray/NotificationsTray.jsx'
import Search from "../Search/Search.jsx";

const Header = () => {

    const [showNotificationsTray, setShowNotificationsTray] = useState(false);

    const toggleNotificationsTray = () => {
        setShowNotificationsTray((prevState) => !prevState);
    }

    return (
        <div className="header__container">
            <header
                className='header__main-content'
                role='banner'
                aria-label='Main Header'
            >
                <h1>meNexus</h1>
            </header>
            <div className="header__search">
                <Search/>
            </div>
            <div className="header__notifications">
                <p
                    className={`header__notifications-tray-toggle ${
                        showNotificationsTray ? 'header__notifications-tray-toggle--active' : ''
                    }`}
                    onClick={toggleNotificationsTray}
                >
                    Notifications
                </p>
                {showNotificationsTray && (
                    <div className="header__notifications-tray">
                        <NotificationsTray/>
                    </div>
                )}
            </div>
        </div>
    )
};

export default Header;