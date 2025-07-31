// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState, useRef } from "react";
import { IoSend } from "react-icons/io5";
import useCreateChatMessage from "../../../api/hooks/useCreateChatMessage.js";
import useChatWebSocket  from '../../../api/hooks/useChatWebSocket.js';

const ChatMessageForm = ({publicKey, activeChannel, sendMessage}) => {
    const [text, setText] = useState(`New Message`);
    const [formClicked, setFormClicked] = useState(false);
    const { createChatMessage, loading, error } = useCreateChatMessage();

    const handleSubmit = () => {
        if (!text.trim()) return;
        sendMessage({
            type: 'chatMessage',
            publicKey: publicKey,
            activeChannel: activeChannel,
            content: text
        });
        setText('');
    };

    const handleFormClick = () => {
        if (!formClicked && text === `New Message`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="message-form flex gap-4 bg-surface p-8 position-fixed-bottom ">
            <div className={`w-full`} onClick={handleFormClick}>
                <textarea
                    className="message-form__entry-field w-full h-full bg-background text-foreground rounded-2xl px-4 py-2"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="message-form__button text-2xl text-foreground " onClick={handleSubmit} disabled={loading}>
                {loading ? "Sending..." : <IoSend />}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
}

export default ChatMessageForm;