// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { FaStar, FaRegThumbsUp, FaTools, FaHandshake, FaCheckCircle } from "react-icons/fa";
import { FaFireFlameCurved } from "react-icons/fa6";


const mockReputation = {
    level: "Trusted",
    score: 1337,
    creator: '63 %',
    curator: '37 %',
    endorsements: 23,
    contributions: 42,
    verifiedCredentials: 3,
};

const UserReputationPanel = () => {
    return (
        <div className="bg-surface rounded-xl shadow-lg p-6 w-full text-foreground border border-border">
            <h2 className="text-2xl font-semibold text-brand mb-4 font-montserrat">Reputation</h2>

            {/* Reputation Level */}
            <div className="flex items-center justify-between mb-4">
                <span className="text-lg text-foreground font-medium font-montserrat">Reputation Level:</span>
                <span className="text-brand font-bold text-md flex items-center gap-2 font-inter">
                    <FaFireFlameCurved className={'text-accent'}/>
                    {mockReputation.level}
                </span>
            </div>

            {/* Score */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral font-montserrat">Reputation Score:</span>
                <span className="font-semibold font-inter">{mockReputation.score}</span>
            </div>
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral font-montserrat">Creator:</span>
                <span className="font-semibold font-inter">{mockReputation.creator}</span>
            </div>
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral font-montserrat">Curator:</span>
                <span className="font-semibold font-inter">{mockReputation.curator}</span>
            </div>

            {/* Endorsements */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral font-montserrat">Peer Endorsements:</span>
                <span className="flex items-center gap-1 font-inter">
                    <FaRegThumbsUp className="text-accent" />
                    {mockReputation.endorsements}
                </span>
            </div>

            {/* Contributions */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral font-montserrat">Verified Contributions:</span>
                <span className="flex items-center gap-1 font-inter">
                    <FaTools className="text-secondary" />
                    {mockReputation.contributions}
                </span>
            </div>

            {/* Credentials */}
            <div className="flex items-center justify-between">
                <span className="text-neutral font-montserrat">Credentials:</span>
                <span className="flex items-center gap-1 font-inter">
                    <FaCheckCircle className="text-green-400" />
                    {mockReputation.verifiedCredentials}
                </span>
            </div>
        </div>
    );
};

export default UserReputationPanel;
