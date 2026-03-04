import api from '../config/axiosConfig';

const serverService = {


    createServer: async (name) => {
        const response = await api.post('/server/createServer', { name });
        return response.data;
    },

    listServers: async () => {
        const response = await api.get('/server/listServers');
        return response.data;
    },

    joinServer: async (serverId) => {
        await api.post(`/server/${serverId}/join`);
    },

    leaveServer: async (serverId) => {
        await api.delete(`/server/${serverId}/leave`);
    },

    listServerMembers: async (serverId) => {
        const response = await api.get(`/server/${serverId}/members`);
        return response.data;
    },

    updateMemberRole: async (serverId, userId, role) => {
        await api.patch(`/server/${serverId}/members/${userId}/role`, { role });
    },

    deleteServer: async (serverId) => {
        await api.delete(`/server/deleteServer/${serverId}`);
    },

    transferOwnership: async (serverId, newOwnerId) => {
        await api.post(`/server/${serverId}/transfer/${newOwnerId}`);
    },

    listChannels: async (serverId) => {
        const response = await api.get(`/server/${serverId}/channels`);
        return response.data;
    },

    createChannel: async (serverId, name) => {
        const response = await api.post(`/server/${serverId}/channels`, { name });
        return response.data;
    },
};

export default serverService;