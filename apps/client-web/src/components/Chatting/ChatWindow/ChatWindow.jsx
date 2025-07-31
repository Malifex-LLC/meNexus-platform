// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ChatMessage from "../ChatMessage/ChatMessage.jsx";

const ChatWindow = ({ publicKey, activeChannel, setActiveChannel, chatMessages, setChatMessages }) => {

    return (
        <div className={'bg-background'}>
            <div className="flex-1 overflow-y-auto p-4  space-y-4 shadow-2xl ">
                {chatMessages.map((msg, i) => {
                    const isOwner = msg.public_key === publicKey;

                    return (
                        <div key={i} className={`flex ${isOwner ? "justify-end" : "justify-start"}`}>
                            <ChatMessage message={msg} isOwner={isOwner} />
                        </div>
                    );
                })}
            </div>
        </div>
    );
}

export default ChatWindow;