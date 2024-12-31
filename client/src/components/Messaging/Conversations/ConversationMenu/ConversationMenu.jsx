import './ConversationMenu.css';
import Conversation from '../Conversation/Conversation.jsx';
import {useState} from "react";

const ConversationMenu = () => {
    const [conversations, setConversations] = useState([]);

    return (
        <div className="conversation-menu">
            <div className="conversation-menu__conversation-list">
                {conversations.length > 0 ? (
                    conversations
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((conversation, index) => (
                            <Conversation
                            key={index}
                            />
                        ))
                ) : (
                    <div> No Conversations Found </div>
                )}
            </div>
        </div>
    )
}

export default ConversationMenu;