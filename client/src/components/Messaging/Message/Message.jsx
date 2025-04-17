import { formatDate } from '../../../utils/dateUtils.js';

const Message = ({
                    session_user_id,
                    conversation_id,
                    message_id,
                    sender_id,
                    content,
                    is_read,
                    created_at,
                 }) => {
    console.log(`session_user_id: ${session_user_id}, sender_id: ${sender_id}, isOwner: ${session_user_id === sender_id}`);
    const isOwner = session_user_id === sender_id;

    return (
        <div className={`flex w-full px-4 mb-8 ${isOwner ? 'justify-end' : 'justify-start'}`}>
            <div
                className={`rounded-2xl px-6 py-4 max-w-xl text-black
          ${isOwner ? 'bg-foreground ' : 'bg-brand '}`}
            >
                <div className="text-sm text-neutral mb-2 text-left">
                    {formatDate(created_at)}
                </div>
                <div className="text-xl">
                    <p>{content}</p>
                </div>
            </div>
        </div>
    );
}

export default Message;