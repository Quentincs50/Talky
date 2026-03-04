import { useEffect, useRef } from "react";

export function useWebSocket(onMessage) {
    const onMessageRef = useRef(onMessage);

    useEffect(() => {
        onMessageRef.current = onMessage;
    });
    

    const wsRef = useRef(null);

    useEffect(() => {
        if (wsRef.current) return;

        const token = localStorage.getItem("token")
        const url = import.meta.env.VITE_API_URL?.replace("http", "ws") || "ws://localhost:3000";
        const ws = new WebSocket(`${url}/ws?token=${token}`);

        ws.onopen = () => console.log("Websocker connecté");
        ws.onclose = () => console.log("Websocker fermé");
        ws.onerror = (e) => console.log("Websocker erreur : ", e);
        ws.onmessage = (event) => onMessageRef.current(JSON.parse(event.data));

        wsRef.current = ws;

        return () => {
            if (ws.readyState === WebSocket.OPEN) {
                ws.close();
            }
            wsRef.current = null;
        };
    }, [])
    return wsRef;
}