import api from '../config/axiosConfig';

const messageService = {

    sendChannelMessage: async (channelId, content) => {
        const response = await api.post(`/api/channels/${channelId}/messages`, {
            content
        });
        return response.data;
    },

    sendPrivateMessage: async (userId, content) => {
        const response = await api.post(`/api/private/${userId}/messages`, {
            content
        });
        return response.data;
    },

    sendServerMessage: async (serverId, content) => {
        const response = await api.post(`/api/servers/${serverId}/messages`, {
            content
        });
        return response.data;
    },

    getChannelMessages: async (channelId) => {
        const response = await api.get(`/api/channels/${channelId}/messages`);
        return response.data;
    },

    getPrivateMessages: async (contactId) => {
        const response = await api.get(`/api/private/${contactId}/messages`);
        return response.data;
    },

    getServerMessages: async (serverId) => {
        const response = await api.get(`/api/servers/${serverId}/messages`);
        return response.data;
    },

    editMessage: async (messageId, content) => {
        const response = await api.patch(`/api/messages/${messageId}`, {
            content
        });
        return response.data;
    },

    deleteMessage: async (messageId) => {
        await api.delete(`/api/messages/${messageId}`);
    }
};

export default messageService;