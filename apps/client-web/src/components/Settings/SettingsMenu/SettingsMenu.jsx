// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { FaKey } from "react-icons/fa";
import { FaDisplay } from "react-icons/fa6";
import { TbLogout2 } from "react-icons/tb";

import { CgProfile } from "react-icons/cg";
import {useNavigate} from "react-router-dom";

const SettingsMenu = ({ onSelectMenu, selectedMenu }) => {
    // TODO Logout doesn't destroy session, just routes to login page
    const navigate = useNavigate();

    return (
        <div className={`p-4 m-8 flex flex-col items-center text-2xl rounded-2xl bg-surface `}>
            <h1 className={`p-4 `}>Settings Menu</h1>
            <div className={`flex h-1 w-full bg-border mb-4`}/>
            <ul className="settings-menu__list ">
                <li onClick={() => onSelectMenu('Profile')}
                    className={`settings - menu__item flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-r-2xl
                ${selectedMenu === 'Profile' ? 'bg-brand' : ''}`}>
                    <CgProfile />
                    Profile
                </li>
                <li
                    onClick={() => onSelectMenu('Account')}
                    className={`settings - menu__item flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-r-2xl
                ${selectedMenu === 'Account' ? 'bg-brand' : ''}`}
                >
                    <FaKey />
                    Account
                </li>
                <li
                    onClick={() => onSelectMenu('Display')}
                    className={`settings - menu__item flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-r-2xl
                ${selectedMenu === 'Display' ? 'bg-brand' : ''}`}>
                    <FaDisplay />
                    Display
                </li>
                <li
                    onClick={() => navigate('/login')}
                    className={`settings - menu__item flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-r-2xl
                ${selectedMenu === 'Logout' ? 'bg-brand' : ''}`}>
                    <TbLogout2 />
                    Logout
                </li>
            </ul>
        </div>

    );
};

export default SettingsMenu;