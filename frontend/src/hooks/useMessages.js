import { useCallback, useEffect, useState } from 'react';
import messageService from '../services/messageService';
import { useWebSocket } from './useWebSocket';

const useMessages = (location, locationId) => {
    const [messages, setMessages] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useWebSocket((data) => {
        const msg = data.message || data;

        const isMatch = location === 'private'
            ? msg.location === 'Private' && (msg.location_id === locationId || msg.owner === locationId)
            : msg.location_id === locationId;

        if (isMatch) {
            setMessages((prev) => {
                if (prev.some((m) => m.id === msg.id)) return prev;
                return [...prev, msg];
            });
        }
    });

    // RETRIEVE MSG BASED ON LOCATION
    const fetchMessages = useCallback(async () => {
        if (!locationId) {
            setLoading(false);
            return;
        }

        try {
            setLoading(true);
            setError(null);

            let response;
            switch (location) {
                case 'channel':
                    response = await messageService.getChannelMessages(locationId);
                    break;
                case 'private':
                    response = await messageService.getPrivateMessages(locationId);
                    break;
                case 'server':
                    response = await messageService.getServerMessages(locationId);
                    break;
                default:
                    throw new Error(`Type de conversation invalide: ${location}`);
            }

            setMessages(response.messages || []);
        } catch (err) {
            console.error('Erreur lors de la récupération des messages:', err);
            setError(err.response?.data?.message || 'Erreur lors du chargement des messages');
        } finally {
            setLoading(false);
        }
    }, [location, locationId]);

    // LOADING MSG
    useEffect(() => {
        fetchMessages();
    }, [fetchMessages]);

    // SEND
    const sendMessage = useCallback(async (content) => {
        if (!content.trim()) {
            return;
        }

        try {
            let response;
            switch (location) {
                case 'channel':
                    response = await messageService.sendChannelMessage(locationId, content);
                    break;
                case 'private':
                    response = await messageService.sendPrivateMessage(locationId, content);
                    break;
                case 'server':
                    response = await messageService.sendServerMessage(locationId, content);
                    break;
                default:
                    throw new Error(`Type de conversation invalide: ${location}`);
            }

            
            return response.message;
        } catch (err) {
            console.error("Erreur lors de l'envoi du message:", err);
            setError(err.response?.data?.message || "Erreur lors de l'envoi du message");
            throw err;
        }
    }, [location, locationId]);

    // UPDATE MSG
    const editMessage = useCallback(async (messageId, newContent) => {
        try {
            const response = await messageService.editMessage(messageId, newContent);

            // UPDATE
            setMessages(prev =>
                prev.map(msg => msg.id === messageId ? response : msg)
            );

            return response;
        } catch (err) {
            console.error('Erreur lors de la modification du message:', err);
            setError(err.response?.data?.message || 'Erreur lors de la modification du message');
            throw err;
        }
    }, []);

    // DELETE
    const deleteMessage = useCallback(async (messageId) => {
        try {
            await messageService.deleteMessage(messageId);

            // TAKE OFF MSG FROM THE LIST
            setMessages(prev => prev.filter(msg => msg.id !== messageId));
        } catch (err) {
            console.error('Erreur lors de la suppression du message:', err);
            setError(err.response?.data?.message || 'Erreur lors de la suppression du message');
            throw err;
        }
    }, []);

    // REFRESH MSG
    const refreshMessages = useCallback(() => {
        fetchMessages();
    }, [fetchMessages]);

    return {
        messages,
        loading,
        error,
        sendMessage,
        editMessage,
        deleteMessage,
        refreshMessages
    };
};

export default useMessages;