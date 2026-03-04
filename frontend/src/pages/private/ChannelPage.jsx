import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import serverService from "../../services/serverService.js";


const ChannelPage = () => {

    const { serverId } = useParams();
    const [channels, setChannels] = useState([]);
    const [loading, setLoading] = useState(true);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchChannels = async () => {
            try {
                const data = await serverService.listChannels(serverId);
                setChannels(data);
            } catch (error) {
                return <div>Erreur lors du chargement des channels, {error}</div>
            } finally {
                setLoading(false);
            }
        };
        fetchChannels();
    }, [serverId])
    if (loading) return <div>Chargement...</div>;

    return (
        <div
            className="flex h-screen items-center justify-center bg-gray-100 dark:bg-neutral-800">
                <div className="flex flex-col items-center gap-4">
                    <h1 className="text-2xl font-bold dark:text-white">Vos channels</h1>
            {channels.map((channel) => (
                <div onClick={() => navigate(`/channel/${channel.id_chan}/messages`)} key={channel.id_chan} className="text-lg dark:text-neutral-200"><p>{channel.name}</p></div>
            ))}
                </div>
                
        </div>
    );
};

export default ChannelPage;