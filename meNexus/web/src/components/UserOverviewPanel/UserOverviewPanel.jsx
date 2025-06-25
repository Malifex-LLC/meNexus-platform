// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import AboutUserPanel from "./AboutUserPanel.jsx";
import UserBadges from "./UserBadges.jsx";
import { FaGlobe, FaGithub, FaSoundcloud } from "react-icons/fa";
import { IoLocationSharp } from "react-icons/io5";
import UserReputationPanel from "../UserReputationPanel/UserReputationPanel.jsx";

const UserOverviewPanel = ({ user }) => {
    return (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 p-4 w-full text-foreground">
            {/* Left Column (2/3) */}
            <div className="flex flex-col gap-6 lg:col-span-2">
                {/* About / Bio */}
                <div className="bg-surface rounded-xl shadow-lg p-6">
                    <h2 className="text-2xl font-semibold text-primary mb-2">About</h2>
                    <p className="text-neutral">{user.bio || "Full-stack engineer and audio tinkerer. Building decentralized tools and modular sound systems."}</p>
                    <div className="flex items-center gap-2 mt-4 text-accent">
                        <IoLocationSharp />
                        <span>{user.location || "Brooklyn, NY"}</span>
                    </div>
                </div>

                {/* External Links */}
                <div className="bg-surface rounded-xl shadow-lg p-6">
                    <h2 className="text-2xl font-semibold text-primary mb-2">Links</h2>
                    <div className="flex flex-col gap-2">
                        <a href="https://malifex.dev" target="_blank" className="flex items-center gap-2 hover:text-brand transition-all">
                            <FaGlobe /> malifex.dev
                        </a>
                        <a href="https://github.com/jacobwileyross" target="_blank" className="flex items-center gap-2 hover:text-brand transition-all">
                            <FaGithub /> github.com/jacobwileyross
                        </a>
                        <a href="https://soundcloud.com/malbloom" target="_blank" className="flex items-center gap-2 hover:text-brand transition-all">
                            <FaSoundcloud /> soundcloud.com/malbloom
                        </a>
                    </div>
                </div>

                {/* Interests / Skills */}
                <div className="bg-surface rounded-xl shadow-lg p-6">
                    <h2 className="text-2xl font-semibold text-primary mb-2">Interests</h2>
                    <div className="flex flex-wrap gap-2">
                        {["Decentralized systems", "Audio programming", "Game dev", "C++", "React", "Synths"].map((tag, i) => (
                            <span key={i} className="bg-primary/10 text-primary px-3 py-1 rounded-full text-sm font-medium">
                                {tag}
                            </span>
                        ))}
                    </div>
                </div>

                {/* Mocked Synapse Memberships */}
                <div className="bg-surface rounded-xl shadow-lg p-6">
                    <h2 className="text-2xl font-semibold text-primary mb-2">Communities</h2>
                    <ul className="text-neutral">
                        <li>🌐 Nexus Builders – Admin</li>
                        <li>🎛️ Modular Audio – Contributor</li>
                        <li>🎮 Game Dev Guild – Member</li>
                    </ul>
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
