import './Conversation.css';
import {useState} from "react";
import Message from '../../../Messaging/Message/Message.jsx'

const Conversation = () => {
    const [messages, setMessages] = useState([]);

    return (
        <div className="conversation">
            <div className="conversation__messages">
                {messages.length > 0 ? (
                    messages
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((message, index) => (
                            <Message
                            key={index}
                            message_id={message.message_id}
                            user_id={message.sender_id}
                            content={message.content}
                            is_read={message.is_read}
                            created_at={message.created_at}
                            />
                        ))
                ) : (
                    <div> No Messages </div>
                )}
            </div>

        </div>
    )
}

export default Conversation;