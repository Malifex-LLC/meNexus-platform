// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ChatMessageForm from "../ChatMessageForm/ChatMessageForm.jsx";
import ChattingChannelsPanel from "../ChattingChannelsPanel/ChattingChannelsPanel.jsx";
import ChatWindow from "../ChatWindow/ChatWindow.jsx";
import useChatWebSocket from "../../../api/hooks/useChatWebSocket.js";

const ChatPanel = ({isLocalSynapse, synapseMetadata, publicKey, channels, activeChannel, setActiveChannel, chatMessages, setChatMessages}) => {

    const { sendMessage } = useChatWebSocket({
        wsUrl: synapseMetadata.identity.webSocketUrl,
        publicKey,
        activeChannel,
        onMessage: (message) => {
            console.log('WebSocket message received:', message);
            setChatMessages(prev => [...prev, message]);
        }
    });

    return (
        <div className="flex flex-col w-full h-full text-foreground bg-surface/70 border border-border xl:rounded-xl ">
            <div className="flex flex-row flex-1 min-h-0  ">
                <div className={'relative flex flex-col w-full'}>
                    <div className="flex-1 h-full overflow-y-auto ">
                        <ChatWindow
                            publicKey={publicKey}
                            activeChannel={activeChannel}
                            setActiveChannel={setActiveChannel}
                            chatMessages={chatMessages}
                            setChatMessages={setChatMessages}
                        />
                    </div>
                    <div className="mt-2 border-t border-border ">
                        <ChatMessageForm
                            isLocalSynapse={isLocalSynapse}
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