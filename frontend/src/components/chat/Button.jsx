import { useState } from 'react';
import { send, add } from "../../assets/index.js";

const MessageButton = ({ onSend }) => {
    const [content, setContent] = useState('');

    const handleSubmit = async (e) => {
        e.preventDefault();

        if (!content.trim()) return;

        try {
            await onSend(content);
            setContent('');
        } catch (error) {
            console.error('Erreur envoi message:', error);
        }
    };

    const handleKeyPress = (e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSubmit(e);
        }
    };

    return (
        <form onSubmit={handleSubmit} className="flex items-center gap-3 w-full">

            <input
                type='text'
                value={content}
                onChange={(e) => setContent(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder='Envoyer un message...'
                className='flex-1 px-6 py-3 bg-gray-100 border border-gray-300 rounded-full
                           focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent
                           placeholder:text-gray-500'
            />

            <button
                type='submit'
                disabled={!content.trim()}
                className='p-3 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-300
                           disabled:cursor-not-allowed rounded-full transition-colors
                           flex items-center justify-center'
            >
                <img src={send} alt='Send' className='w-5 h-5 brightness-0 invert' />
            </button>

            <button
                type='button'
                className='p-3 bg-gray-200 hover:bg-gray-300 rounded-full transition-colors
                           flex items-center justify-center'
            >
                <img src={add} alt='Add' className='w-5 h-5' />
            </button>
        </form>
    );
};

export default MessageButton;