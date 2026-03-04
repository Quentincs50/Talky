import { useRef, useEffect } from 'react';
import Bubble from "./Bubble.jsx";


export default function Conversation({ messages, currentUserId, loading }) {
    const messagesEndRef = useRef(null);
    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    };
    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    if (loading) {
        return (
            <div className='h-[75%] border w-full flex items-center justify-center'>
                <p>Chargement des messages...</p>
            </div>
        );
    }

    return (
        <div className='h-[75%] w-full overflow-y-scroll'>
            <div className='h-full px-10 py-14'>
                {messages.length === 0 ? (
                    <p className='text-center text-gray-500'>Aucun message</p>
                ) : (
                    messages.map((message) => (
                        <Bubble
                            key={message.id || index}
                            message={message}
                            isUser={message.owner === currentUserId}
                        />
                    ))
                )}
                {/* AUTO SCROLL*/}
                <div ref={messagesEndRef} />
            </div>
        </div>
    )

}