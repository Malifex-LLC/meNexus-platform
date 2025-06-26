// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { FaStar, FaRegThumbsUp, FaTools, FaHandshake, FaCheckCircle } from "react-icons/fa";
import { FaFireFlameCurved } from "react-icons/fa6";


const mockReputation = {
    level: "Core Maintainer",
    score: 812,
    endorsements: 23,
    contributions: 42,
    verifiedCredentials: 3,
};

const UserReputationPanel = () => {
    return (
        <div className="bg-surface rounded-xl shadow-lg p-6 w-full text-foreground border border-border">
            <h2 className="text-2xl font-semibold text-primary mb-4">Reputation</h2>

            {/* Reputation Level */}
            <div className="flex items-center justify-between mb-4">
                <span className="text-lg font-medium">Reputation Level:</span>
                <span className="text-brand font-bold text-md flex items-center gap-2">
                    <FaFireFlameCurved className={'text-accent'}/>
                    {mockReputation.level}
                </span>
            </div>

            {/* Score */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral">Reputation Score:</span>
                <span className="font-semibold">{mockReputation.score}</span>
            </div>

            {/* Endorsements */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral">Peer Endorsements:</span>
                <span className="flex items-center gap-1">
                    <FaRegThumbsUp className="text-accent" />
                    {mockReputation.endorsements}
                </span>
            </div>

            {/* Contributions */}
            <div className="flex items-center justify-between mb-3">
                <span className="text-neutral">Verified Contributions:</span>
                <span className="flex items-center gap-1">
                    <FaTools className="text-secondary" />
                    {mockReputation.contributions}
                </span>
            </div>

            {/* Credentials */}
            <div className="flex items-center justify-between">
                <span className="text-neutral">Credentials:</span>
                <span className="flex items-center gap-1">
                    <FaCheckCircle className="text-green-400" />
                    {mockReputation.verifiedCredentials}
                </span>
            </div>
        </div>
    );
};

export default UserReputationPanel;
