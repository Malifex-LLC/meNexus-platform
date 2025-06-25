// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import MessagesLayout from '../../layouts/MessagesLayout/MessagesLayout.jsx';
import ConversationMenu from "../../components/Messaging/Conversations/ConversationMenu/ConversationMenu.jsx";
import Conversation from '../../components/Messaging/Conversations/Conversation/Conversation.jsx'
import {useState} from "react";
import useGetConversations from "../../api/hooks/useGetConversations.js";

const MessagesPage = () => {
    const [selectedConversation, setSelectedConversation] = useState(null);
    const [conversations, setConversations] = useState([]);
    const [activeConversationId, setActiveConversationId] = useState(null);

    const { getConversations } = useGetConversations();

    return (
        <MessagesLayout>
            <ConversationMenu
                getConversations={getConversations}
                setSelectedConversation={setSelectedConversation}
                conversations={conversations}
                setConversations={setConversations}
                activeConversationId={activeConversationId}
                setActiveConversationId={setActiveConversationId}
            />
            {selectedConversation ? (
                    <Conversation
                        conversation_id={selectedConversation.conversation_id}
                        getConversations={getConversations}
                        setConversations={setConversations}
                        setActiveConversationId={setActiveConversationId}
                        setSelectedConversation={setSelectedConversation}
                        participant_id={selectedConversation.participant_id}

                    />

            ) : (
                <div className="flex flex-1 items-center justify-center text-foreground">
                    <p>Select a conversation to view messages</p>
                </div>
            )}
        </MessagesLayout>
    );
};

export default MessagesPage;