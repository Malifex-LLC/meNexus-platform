import { useEffect, useRef } from 'react';

const useActivityWebSocket = ({ wsUrl, publicKey, onActivity }) => {
    const socketRef = useRef(null);

    useEffect(() => {
        const socket = new WebSocket(`${wsUrl}?publicKey=${publicKey}`);
        socketRef.current = socket;

        socket.onopen = () => {
            console.log('Activity WebSocket connected');
            // Optional: send subscribe event
            socket.send(JSON.stringify({ type: "subscribeToActivity", publicKey }));
        };

        socket.onmessage = (event) => {
            const data = JSON.parse(event.data);
            console.log("Activity WebSocket received:", data);

            if (data.type === 'newActivity') {
                onActivity(data.activity);
            }
        };

        socket.onclose = () => {
            console.log('Activity WebSocket disconnected');
        };

        return () => {
            socket.close();
        };
    }, [publicKey, wsUrl]);

    return {};
};

export default useActivityWebSocket;
