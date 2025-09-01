// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { FaLink, FaClock } from "react-icons/fa";

const mockShowcaseItems = [
    {
        title: "meNexus",
        description: "A peer-to-peer social layer for the open internet.",
        link: "https://github.com/Malifex-LLC/meNexus-platform",
        timestamp: "Jan 2025",
    },
    {
        title: "Malifex.dev",
        description: "Personal site and studio portal for my work in decentralized systems, sound, and design.",
        link: "https://malifex.dev",
        timestamp: "Mar 2025",
    },
];

const UserShowcasePanel = () => {
    return (
        <div className="p-6 w-full text-foreground">
            <h2 className="text-3xl text-brand font-semibold mb-6">Showcase</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                {mockShowcaseItems.map((item, index) => (
                    <a
                        key={index}
                        href={item.link}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="flex items-center gap-2 bg-surface border border-border rounded-xl p-5 shadow-md hover:shadow-lg transition-shadow duration-300"
                    >
                        <div className="">
                            <h3 className="flex gap-2 text-xl font-semibold text-brand mb-2"><FaLink /> {item.title}</h3>
                            <p className="text-neutral mb-4 line-clamp-3">{item.description}</p>
                            <div className="flex justify-between items-center text-sm text-muted">
                            <span className="flex items-center gap-2 text-neutral">
                                <FaClock /> {item.timestamp}
                            </span>
                            </div>
                        </div>
                    </a>

                ))}
            </div>
        </div>
    );
};

export default UserShowcasePanel;
