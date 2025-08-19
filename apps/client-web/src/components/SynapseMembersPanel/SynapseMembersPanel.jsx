// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ProfileCard from "../ProfileCard/ProfileCard.jsx";
import {useEffect} from "react";

const SynapseMembersPanel = ({ members }) => {

    if (!members) {
        return <div>Loading Synapse members...</div>
    }

    if (members.length === 0) {
        return <div className={'text-foreground text-center text-xl p-4'}>No Synapse members...</div>
    }

    return (
        <div>
            {members.map(member => {
                return (
                    <ProfileCard
                        key={member.public_key}
                        publicKey={member.public_key}
                    />
                )
            })}
        </div>
    )
}

export default SynapseMembersPanel;