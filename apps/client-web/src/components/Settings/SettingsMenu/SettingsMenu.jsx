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
        <div className={`flex flex-col m-8 items-center text-2xl rounded-xl bg-surface font-montserrat`}>
            <ul className="p-4">
                <li onClick={() => onSelectMenu('Profile')}
                    className={`flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-xl
                ${selectedMenu === 'Profile' ? 'bg-brand/60' : 'hover:bg-brand/10 hover:cursor-pointer'}`}>
                    <CgProfile />
                    Profile
                </li>
                <li
                    onClick={() => onSelectMenu('Account')}
                    className={`flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-xl
                ${selectedMenu === 'Account' ? 'bg-brand/60' : 'hover:bg-brand/10 hover:cursor-pointer'}`}
                >
                    <FaKey />
                    Account
                </li>
                <li
                    onClick={() => onSelectMenu('Display')}
                    className={`flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-xl
                ${selectedMenu === 'Display' ? 'bg-brand/60' : 'hover:bg-brand/10 hover:cursor-pointer'}`}>
                    <FaDisplay />
                    Display
                </li>
                <li
                    onClick={() => navigate('/login')}
                    className={`flex gap-4 px-16 py-4 mb-8 w-full items-center rounded-xl
                ${selectedMenu === 'Logout' ? 'bg-brand/60' : 'hover:bg-brand/10 hover:cursor-pointer'}`}>
                    <TbLogout2 />
                    Logout
                </li>
            </ul>
        </div>

    );
};

export default SettingsMenu;