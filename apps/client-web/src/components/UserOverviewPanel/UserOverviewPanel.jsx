// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import UserAbout from "./UserAbout.jsx";
import UserBadges from "./UserBadges.jsx";
import { FaGlobe, FaGithub, FaSoundcloud } from "react-icons/fa";
import { IoLocationSharp } from "react-icons/io5";
import UserReputationPanel from "../UserReputationPanel/UserReputationPanel.jsx";
import UserInterests from "./UserInterests.jsx";
import UserLinks from "./UserLinks.jsx";

const UserOverviewPanel = ({ user }) => {
    return (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 p-4 w-full text-foreground">
            {/* Left Column (2/3) */}
            <div className="flex flex-col gap-6 lg:col-span-2">
                {/* About / Bio */}
                <div className="">
                    <UserAbout user={user} />
                </div>

                {/* External Links */}
                <div>
                    <UserLinks/>
                </div>

                {/* Interests */}
                <div>
                    <UserInterests/>
                </div>
            </div>

            {/* Right Column (1/3) */}
            <div className="flex flex-col gap-6">
                <UserBadges />
                <UserReputationPanel />
            </div>
        </div>
    );
};

export default UserOverviewPanel;
