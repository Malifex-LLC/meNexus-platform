import { ENDPOINTS } from "../../config/endpoints.js"

const useMessagesWebSocket = () => {

    const connectMessagesWebSocket = (user_id, onMessageReceived) => {
        if (!user_id) return;

        let ws;
        let reconnectTimeout;

        const connectWebSocket = () => {
            ws = new WebSocket(`${ENDPOINTS.WEBSOCKET}${user_id}`);
            ws.onopen = () => {
                console.log("WebSocket connected");
            };
            ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                onMessageReceived(message);
            };
            ws.onclose = () => {
                console.log("WebSocket connection closed. Reconnecting...");
                reconnectTimeout = setTimeout(connectWebSocket, 5000); // Retry every 5 seconds
            };
            ws.onerror = (error) => {
                console.error("WebSocket error:", error);
            };
        };

        connectWebSocket();
        return () => {
            clearTimeout(reconnectTimeout);
            ws.close(); // Clean up on component unmount
        };

    };

    return {
        connectMessagesWebSocket,
    };
};

export default useMessagesWebSocket;