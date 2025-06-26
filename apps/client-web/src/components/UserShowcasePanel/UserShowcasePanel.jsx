// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { FaLink, FaClock } from "react-icons/fa";

const mockShowcaseItems = [
    {
        title: "Malifex.dev",
        description: "Personal site and studio portal for my work in decentralized systems, sound, and design.",
        link: "https://malifex.dev",
        timestamp: "Mar 2025",
    },
    {
        title: "Bloom Engine",
        description: "Bloom Engine is a 2D game engine built in C++ utilizing SDL2, Dear ImGui, FMOD, and Lua.",
        link: "https://github.com/JacobWileyRoss/Bloom_Engine",
        timestamp: "Jan 2025",
    },
    {
        title: "Gardens & Villa - Orange Blossom (Malbloom Bootleg)",
        description: "A bootleg remix I did of a favorite track from a favorite band. Indie Dance kinda vibes.",
        link: "https://soundcloud.com/malbloom/gardens-villa-orange-blossom",
        timestamp: "Nov 2024",
    },
];

const UserShowcasePanel = () => {
    return (
        <div className="p-6 w-full text-foreground">
            <h2 className="text-3xl text-primary font-semibold mb-6">Showcase</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                {mockShowcaseItems.map((item, index) => (
                    <div key={index} className="bg-surface border border-border rounded-xl p-5 shadow-md hover:shadow-lg transition-shadow duration-300">
                        <h3 className="text-xl font-semibold text-brand mb-2">{item.title}</h3>
                        <p className="text-neutral mb-4 line-clamp-3">{item.description}</p>
                        <div className="flex justify-between items-center text-sm text-muted">
                            <a href={item.link} target="_blank" rel="noopener noreferrer" className="flex items-center gap-2 hover:text-accent">
                                <FaLink /> Visit
                            </a>
                            <span className="flex items-center gap-2">
                                <FaClock /> {item.timestamp}
                            </span>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default UserShowcasePanel;
