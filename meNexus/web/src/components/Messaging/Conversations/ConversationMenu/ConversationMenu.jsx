import './ConversationMenu.css';
import {useEffect} from "react";
import useCreateConversation from "../../../../hooks/api/useCreateConversation.js";
import useSetMessagesAsRead from "../../../../hooks/api/useSetMessagesAsRead.js";

const ConversationMenu = ({
                              getConversations,
                              setSelectedConversation,
                              conversations,
                              setConversations,
                              activeConversationId,
                              setActiveConversationId
}) => {
    const { createConversation } = useCreateConversation();
    const { setMessagesAsRead } = useSetMessagesAsRead();

    // Fetch Conversations data
    useEffect(() => {
        const fetchConversations = async () => {
            try {
                console.log("Fetching conversations");
                const conversationsData = await getConversations();
                setConversations(conversationsData);

                const hasUnread = conversationsData.some(conversation => conversation.has_unread_messages);
                console.log("hasUnread: ", hasUnread);

            } catch (error) {
                console.error('Error fetching conversations', error);
            }
        };

        fetchConversations();
    },[]);

    const handleConversationClick = async (conversation) => {
        setActiveConversationId(conversation.conversation_id); // Set the active conversation
        setSelectedConversation(conversation); // Notify parent component
        await setMessagesAsRead(conversation.conversation_id);

        // Refresh conversations
        const refreshedConversations = await getConversations();
        setConversations(refreshedConversations);
    };

    const handleComposeClick = async () => {
        try {
            const newConversation = await createConversation();
            console.log("newConversationId:", newConversation.conversation_id);
            setConversations((prev) => [newConversation, ...prev]);
            setActiveConversationId(newConversation.conversation_id);
            setSelectedConversation(newConversation);
        } catch (error) {
            console.error('Error creating conversation', error);
        }
    };

    return (
        <div className="conversation-menu">
            <div className="conversation-menu__new-conversation" onClick={handleComposeClick}>
                Compose
            </div>
            <div className="conversation-menu__conversation-list">
                {conversations.length > 0 ? (
                    conversations
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((conversation) => (
                            <div
                                key={conversation.conversation_id}
                                className={`conversation-menu__conversation ${
                                    activeConversationId === conversation.conversation_id ? 'active' : ''
                                }${conversation.has_unread_messages ? 'conversation-menu__conversation--unread' : ''}`}
                                onClick={() => handleConversationClick(conversation)}
                            >
                                <p>{conversation.participant_handle}</p>
                            </div>
                        ))
                ) : (
                    <div>No Conversations Found</div>
                )}
            </div>
        </div>
    );
}

export default ConversationMenu;