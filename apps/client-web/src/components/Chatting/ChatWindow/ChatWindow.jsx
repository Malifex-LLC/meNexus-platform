// ChatWindow.jsx

import { useEffect, useRef } from "react";
import ChatMessage from "../ChatMessage/ChatMessage.jsx";

const ChatWindow = ({ publicKey, chatMessages }) => {
    const containerRef = useRef(null);
    const bottomRef = useRef(null);

    useEffect(() => {
        const timeout = setTimeout(() => {
            bottomRef.current?.scrollIntoView({ behavior: "smooth" });
        }, 1000); // allow DOM/layout to fully settle

        return () => clearTimeout(timeout);
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
