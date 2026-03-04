const Bubble = ({message, isUser}) => {
    return (
        <div className={`max-w-[40%] ${isUser ? 'bg-green-500 ml-auto rounded-bl-xl rounded-t-xl' : 'bg-blue-500 rounded-b-xl rounded-tr-xl'}  p-4 mb-6`}>
            <p>{message.content}</p>
            <span>
                {new Date(message.createdAt).toLocaleTimeString('fr-FR', {
                    hour: '2-digit',
                    minute: '2-digit'
                })}
            </span>
        </div>
    )
}
export default Bubble;