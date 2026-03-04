import { useEffect, useState, useCallback } from "react";
import api from "../config/axiosConfig";

export default function useParticipants(type, id = null) {
    const [participants, setParticipants] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    const fetchParticipants = useCallback(async () => {
        setLoading(true);
        setError(null);

        try {
            let res;

            switch (type) {
                case "contacts":
                    res = await api.get(`auth/me/`);
                    break;
                case "server":
                    if (!id) throw new Error("serverId required");
                    res = await api.get(`server/{server_id}/members`);
                    break;
                case "channel":
                    if (!id) throw new Error("channelId required");
                    res = await api.get(`channel/{channel_id}`);
                    break;
                default:
                    throw new Error(`Type invalide: ${type}`);
            }

            setParticipants(res.data);
        } catch (err) {
            console.error("Erreur participants:", err);
            setError(err.response?.data?.message || err.message);
        } finally {
            setLoading(false);
        }
    }, [type, id]);

    useEffect(() => {
        fetchParticipants();
    }, [fetchParticipants]);

    return { participants, loading, error, refresh: fetchParticipants };
}
