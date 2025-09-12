// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Channel from '../ChattingChannel/Channel.jsx';
import {useState} from "react";

const ChattingChannelsPanel = ({channels, activeChannel, setActiveChannel}) => {

    return (
        <div className={'flex flex-col p-4 w-full'}>
            <div className="text-foreground text-center text-xl border-b border-border">
                Channels
            </div>

            {channels.map((channel) => (
                <button
                    key={channel}
                    onClick={() => setActiveChannel(channel)}
                    className={`
            rounded-xl p-2 mt-4 w-full text-left text-xl text-foreground border border-border rounded-xl shadow-lg 
            hover:bg-brand/10 hover:translate-y-[-2px]
            ${activeChannel === channel ? "bg-surface text-background  " : "bg-background"}
          `}
                >
                    <Channel channel={channel} />
                </button>
            ))}
        </div>
    );
};


export default ChattingChannelsPanel;