// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import {useEffect} from "react";
import useCreateConversation from "../../../../api/hooks/useCreateConversation.js";
import useSetMessagesAsRead from "../../../../api/hooks/useSetMessagesAsRead.js";
import { PiNotePencil } from "react-icons/pi";


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
        <div className="conversation-menu  text-foreground text-2xl bg-surface rounded-2xl p-4 mx-4 flex flex-col h-full ">
            <div className="conversation-menu__new-conversation flex justify-center gap-4 m-4 text-3xl "
                 onClick={handleComposeClick}>
                New Message
                <PiNotePencil />
            </div>
            <div className={`w-full h-1 bg-border mb-4 `}/>
            <div className="conversation-menu__conversation-list h-screen  overflow-y-auto ">
                {conversations.length > 0 ? (
                    conversations
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((conversation) => (
                            <div
                                key={conversation.conversation_id}
                                className={`conversation-menu__conversation  p-8 mx-4 rounded-xl ${
                                    activeConversationId === conversation.conversation_id ? 'active bg-brand text-black' : ''
                                }${conversation.has_unread_messages ? 'conversation-menu__conversation--unread bg-red-700' : ''}`}
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