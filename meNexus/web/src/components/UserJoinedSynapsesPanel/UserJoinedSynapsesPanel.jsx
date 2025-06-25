// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";

const UserJoinedSynapsesPanel = ({user}) => {
    return (
        <div>
            <JoinedSynapsesPanel synapses={user.synapses} />
        </div>
    );
}

export default UserJoinedSynapsesPanel;