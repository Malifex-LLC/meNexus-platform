// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ChatMessageForm from "../ChatMessageForm/ChatMessageForm.jsx";
import ChattingChannelsPanel from "../ChattingChannelsPanel/ChattingChannelsPanel.jsx";
import ChatWindow from "../ChatWindow/ChatWindow.jsx";
import useChatWebSocket from "../../../api/hooks/useChatWebSocket.js";

const ChatPanel = ({isLocalSynapse, publicKey, synapsePublicKey, channels, activeChannel, setActiveChannel, chatMessages, setChatMessages}) => {

    const { sendMessage } = useChatWebSocket({
        publicKey,
        activeChannel,
        onMessage: (message) => {
            console.log('WebSocket message received:', message);
            setActiveChannel(message.activeChannel);
            setChatMessages(prev => [message, ...prev]);
        }
    });

    return (
        <div className="flex flex-col w-full h-full text-foreground  ">
            <div className="flex flex-row flex-1 min-h-0 p-4 gap-4 ">
                <div className="w-1/5 h-full overflow-y-auto p-4 border border-border shadow-2xl rounded-xl bg-background">
                    <ChattingChannelsPanel
                        channels={channels}
                        activeChannel={activeChannel}
                        setActiveChannel={setActiveChannel}
                    />
                </div>
                <div className={'relative flex flex-col w-full'}>
                    <div className="flex-1 h-full overflow-y-auto border border-border shadow-lg rounded-xl">
                        <ChatWindow
                            publicKey={publicKey}
                            activeChannel={activeChannel}
                            setActiveChannel={setActiveChannel}
                            chatMessages={chatMessages}
                            setChatMessages={setChatMessages}
                        />
                    </div>
                    <div className="p-4 mt-2 border border-border bg-background shadow-2xl rounded-xl">
                        <ChatMessageForm
                            publicKey={publicKey}
                            activeChannel={activeChannel}
                            sendMessage={sendMessage}
                        />
                    </div>
                </div>
            </div>
        </div>
    );

}

export default ChatPanel;