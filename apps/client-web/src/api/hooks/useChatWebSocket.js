import { useEffect, useRef } from 'react';

const useChatSocket = ({ publicKey, activeChannel, onMessage }) => {
    const socketRef = useRef(null);

    useEffect(() => {
        const socket = new WebSocket(`${import.meta.env.VITE_WS_BASE_URL}?publicKey=${publicKey}`);
        socketRef.current = socket;

        socket.onopen = () => {
            console.log('WebSocket connected');
        };

        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            if (data.type === 'newChatMessage' && data.activeChannel === activeChannel) {
                onMessage(data.message);
            }
        };

        socket.onclose = () => {
            console.log('WebSocket disconnected');
        };

        return () => {
            socket.close();
        };
    }, [publicKey]);

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
