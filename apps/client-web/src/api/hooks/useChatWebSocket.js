import { useEffect, useRef } from 'react';

const useChatSocket = ({ wsUrl, publicKey, activeChannel, onMessage }) => {
    const socketRef = useRef(null);
    const activeChannelRef = useRef(activeChannel);

    useEffect(() => {
        activeChannelRef.current = activeChannel;
    }, [activeChannel]);

    useEffect(() => {
        const socket = new WebSocket(`${wsUrl}?publicKey=${publicKey}`);
        socketRef.current = socket;

        socket.onopen = () => {
            console.log('Chat WebSocket connected');

            // Optional: send setChannel to server if you want server-side filtering
            socket.send(JSON.stringify({
                type: "setChannel",
                publicKey,
                activeChannel: activeChannelRef.current
            }));
        };

        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            console.log("WebSocket received:", data);

            if (data.type === 'newChatMessage' && data.activeChannel === activeChannelRef.current) {
                onMessage(data.message);
            }
        };

        socket.onclose = () => {
            console.log('WebSocket disconnected');
        };

        return () => {
            socket.close();
        };
    }, [publicKey, wsUrl]); // Only re-run if publicKey changes

    // Re-send activeChannel when it changes
    useEffect(() => {
        if (socketRef.current?.readyState === WebSocket.OPEN) {
            socketRef.current.send(JSON.stringify({
                type: "setChannel",
                publicKey,
                activeChannel
            }));
        }
    }, [activeChannel]);

    const sendMessage = (msg) => {
        if (socketRef.current?.readyState === WebSocket.OPEN) {
            socketRef.current.send(JSON.stringify(msg));
        }
    };

    return {
        sendMessage,
    };
};

export default useChatSocket;
