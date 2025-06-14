import { ENDPOINTS } from "../config.js"

const useNotificationsWebSocket = () => {

    const connectNotificationsWebSocket = (user_id, onNotificationReceived) => {
        if (!user_id) return;

        let ws;
        let reconnectTimeout;

        const connectWebSocket = () => {
            ws = new WebSocket(`${ENDPOINTS.WEBSOCKET}?user_id=${user_id}`);
            ws.onopen = () => {
                console.log("WebSocket connected");
            };
            ws.onmessage = (event) => {
                const notification = JSON.parse(event.data);
                onNotificationReceived(notification);
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
        connectNotificationsWebSocket,
    };
};

export default useNotificationsWebSocket;
