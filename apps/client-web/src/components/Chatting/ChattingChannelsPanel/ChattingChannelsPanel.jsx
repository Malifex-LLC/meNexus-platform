// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Channel from '../ChattingChannel/Channel.jsx';
import {useState} from "react";

const ChattingChannelsPanel = ({channels, activeChannel, setActiveChannel}) => {

    return (
        <div className={''}>
            <div className="text-center rounded-xl border-b border-border">
                Channels
            </div>

            {channels.map((channel) => (
                <button
                    key={channel}
                    onClick={() => setActiveChannel(channel)}
                    className={`
            rounded-xl p-2 mt-4 w-full text-left text-xl shadow-lg
            hover:bg-surface hover:translate-y-[-2px]
            ${activeChannel === channel ? "bg-primary text-background  " : "bg-background"}
          `}
                >
                    <Channel channel={channel} />
                </button>
            ))}
        </div>
    );
};


export default ChattingChannelsPanel;