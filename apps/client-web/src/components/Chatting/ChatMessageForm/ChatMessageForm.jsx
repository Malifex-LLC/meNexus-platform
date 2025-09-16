// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, { useEffect, useState, useRef } from "react";
import { IoSend } from "react-icons/io5";
import useCreateChatMessage from "../../../api/hooks/useCreateChatMessage.js";
import useChatWebSocket  from '../../../api/hooks/useChatWebSocket.js';
import { TbGif } from "react-icons/tb";
import GiphyTray from "../../GIPHY/GiphyTray.jsx";


const ChatMessageForm = ({isLocalSynapse, publicKey, activeChannel, sendMessage}) => {
    const [text, setText] = useState(`New Message`);
    const [formClicked, setFormClicked] = useState(false);
    const [isGiphyTrayOpen, setIsGiphyTrayOpen] = useState(false);
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

    const toggleGiphyTray = () => {
        setIsGiphyTrayOpen((prevState) => !prevState);

    }

    // Close drawers on ESC
    useEffect(() => {
        if (!(isGiphyTrayOpen)) return;
        const onKey = (e) => e.key === 'Escape' && (setIsGiphyTrayOpen(false), setIsGiphyTrayOpen(false));
        window.addEventListener('keydown', onKey);
        return () => window.removeEventListener('keydown', onKey);
    }, [isGiphyTrayOpen]);

    return (
        <div className="flex flex-col w-full h-full relative">

            {isGiphyTrayOpen && (

                <div className="absolute bottom-full  left-0 right-0 z-50">
                    <div className="rounded-t-xl border border-border shadow-2xl max-h-[60vh] overflow-auto">
                        <GiphyTray
                            isLocalSynapse={isLocalSynapse}
                            publicKey={publicKey}
                            activeChannel={activeChannel}
                            sendMessage={sendMessage}
                            toggleGiphyTray={toggleGiphyTray}
                        />
                    </div>
                </div>
            )}
            {/* ===== Mobile Backdrop (shared) ===== */}
            {(isGiphyTrayOpen) && (
                <button
                    className="fixed inset-0 z-[49] "
                    aria-label="Close menus"
                    onClick={() => { setIsGiphyTrayOpen(false); setIsGiphyTrayOpen(false); }}
                />
            )}
            <div className={`flex gap-4 bg-background p-2 xl:p-4 position-fixed-bottom `}>
                <div className={`w-full`} onClick={handleFormClick}>
                <textarea
                    className="w-full h-full bg-surface text-foreground rounded-xl px-4 py-2 focus:outline-1 focus:outline-brand/60"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault();
                            handleSubmit();
                        }
                    }}
                />
                </div>
                <button
                    className={`text-foreground text-4xl hover:text-brand/60 hover:cursor-pointer`}
                    onClick={toggleGiphyTray}
                >
                    <TbGif />
                </button>
                <button className="message-form__button text-2xl text-foreground " onClick={handleSubmit} disabled={loading}>
                    {loading ? "Sending..." : <IoSend />}
                </button>
                {error && <div className="error">Error: {error}</div>}
            </div>
        </div>
    );
}

export default ChatMessageForm;