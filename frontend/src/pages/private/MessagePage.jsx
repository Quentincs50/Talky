import { useParams } from "react-router-dom";
import Bubble from "../../components/chat/Bubble";
import MessageButton from "../../components/chat/Button";
import { useAuth } from "../../hooks/useAuth";
import useMessages from "../../hooks/useMessages";

export default function MessagePage() {
    const { id } = useParams();
    const { user } = useAuth();
    const { messages, loading, sendMessage } = useMessages('channel', id)

    if (loading) return <div>Chargement...</div>;

    

    return(
        <div>
            <h1 className="text-lg dark:text-neutral-200">Vos messages</h1>
            {messages.map((message) => (
                <Bubble key={message.id} message={message} isUser={message.owner === user?.id} />
            ))}

            <div className="p-6 bg-white">
                <MessageButton onSend={sendMessage}/>
            </div>
        </div>
    )
}