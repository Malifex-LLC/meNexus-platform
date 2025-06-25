import { formatDate } from '../../../utils/dateUtils.js';

const Message = ({
                    sessionPublicKey,
                    conversation_id,
                    message_id,
                    sender_public_key,
                    content,
                    is_read,
                    created_at,
                 }) => {
    console.log(`sessionPublicKey: ${sessionPublicKey}, sender_id: ${sender_public_key}, isOwner: ${sessionPublicKey === sender_public_key}`);
    const isOwner = sessionPublicKey === sender_public_key;

    return (
        <div className={`flex w-full px-4 mb-8 ${isOwner ? 'justify-end' : 'justify-start'}`}>
            <div
                className={`rounded-2xl px-6 py-4 max-w-xl 
          ${isOwner ? 'bg-brand text-foreground-message' : 'bg-surface text-foreground '}`}
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