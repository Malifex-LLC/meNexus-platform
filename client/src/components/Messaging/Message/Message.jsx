import './Message.css';
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
        <div className={`message ${isOwner ? 'message--owner' : 'message--other'}`}>
            <div className="message__date">
                <p>{formatDate(created_at)}</p>
            </div>
            <div className="message__main-content">
                <p>{content}</p>
            </div>
        </div>
    )
}

export default Message;