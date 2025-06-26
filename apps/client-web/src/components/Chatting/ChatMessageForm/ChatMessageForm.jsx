// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useState} from "react";
import useCreateMessage from "../../../api/hooks/useCreateMessage.js";
import {IoSend} from "react-icons/io5";

const ChatMessageForm = () => {
    const [text, setText] = useState(`New Message`);
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createMessage, loading, error } = useCreateMessage(() => refreshMessages());

    const handleSubmit = async () => {
        if (!text.trim()) return; // Avoid sending empty messages
        const message = {
            participant_id: participant_id,
            content: text,
        };
        console.log("Submitting message:", message);
        await createMessage(conversation_id, message);
        setText(""); // Reset the text field after submission
        await refreshMessages(getMessages, conversation_id, setMessages); // Refresh messages
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