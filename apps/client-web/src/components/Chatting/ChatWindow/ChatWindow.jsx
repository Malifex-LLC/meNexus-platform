// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useRef } from "react";
import ChatMessage from "../ChatMessage/ChatMessage.jsx";

const ChatWindow = ({ publicKey, chatMessages }) => {
    const containerRef = useRef(null);
    const bottomRef = useRef(null);

    useEffect(() => {
        let frame1, frame2;

        // Wait 2 frames to ensure layout is flushed
        frame1 = requestAnimationFrame(() => {
            frame2 = requestAnimationFrame(() => {
                bottomRef.current?.scrollIntoView({ behavior: "smooth" });
            });
        });

        return () => {
            cancelAnimationFrame(frame1);
            cancelAnimationFrame(frame2);
        };
    }, [chatMessages]);



    return (
        <div
            ref={containerRef}
            className="flex flex-col flex-grow overflow-y-auto p-4 space-y-4"
        >
            {chatMessages.map((msg, i) => {
                const isOwner = msg.public_key === publicKey;
                return (
                    <div key={i} className={`flex ${isOwner ? "justify-end" : "justify-start"}`}>
                        <ChatMessage message={msg} isOwner={isOwner} />
                    </div>
                );
            })}
            <div ref={bottomRef}></div>
        </div>
    );
};

export default ChatWindow;