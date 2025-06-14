import {useEffect, useState} from "react";
import useGetSessionUser from '../../../../api/hooks/useGetSessionUser.js'
import Message from '../../Message/Message.jsx'
import MessageForm from '../../MessageForm/MessageForm.jsx'
import useGetMessages from '../../../../api/hooks/useGetMessages.js'
import useMessagesWebSocket from '../../../../api/hooks/useMessagesWebSocket.js'
import { refreshMessages } from '../../../../utils/apiUtils.js'
import useUpdateConversationParticipants from "../../../../api/hooks/useUpdateConversationParticipants.js";
import useSetMessagesAsRead from "../../../../api/hooks/useSetMessagesAsRead.js";

const Conversation = ({
                          conversation_id,
                          getConversations,
                          setConversations,
                          setActiveConversationId,
                          setSelectedConversation,
                          participant_id
}) => {
    const [sessionUserId, setSessionUserId] = useState(null);
    const [isSessionUserIdSet, setIsSessionUserIdSet] = useState(null);
    const [participants, setParticipants] = useState( "");
    const [messages, setMessages] = useState([]);
    const [handleSaveParticipantsError, setHandleSaveParticipantsError] = useState(false);

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getMessages, loading, error } = useGetMessages();
    const { connectMessagesWebSocket } = useMessagesWebSocket();
    const { updateConversationParticipants } = useUpdateConversationParticipants();
    const { setMessagesAsRead } = useSetMessagesAsRead();

    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!sessionUserId && !isSessionUserIdSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();
                    console.log(response);

                    if (response.status === 200 && response.data.user_id) {
                        console.log("Session user handle:", response.data.handle);
                        setSessionUserId(response.data.user_id);
                    } else {
                        console.error("Invalid session");
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                }
            } else if (sessionUserId) {
                setSessionUserId(sessionUserId);
            }
        };

        fetchSessionUser();
    }, [sessionUserId, isSessionUserIdSet]);

    // Fetch Messages data
    useEffect(() => {
        const fetchMessages = async () => {
            try {
                console.log("Fetching messages");
                const messagesData = await getMessages(conversation_id);
                setMessages(messagesData);
                console.log('Fetched messagesData:', messagesData);
            } catch (error) {
                console.error('Error fetching messages', error);
            }
        };

        fetchMessages();
    }, [conversation_id]);

    const handleSaveParticipants = async () => {
        try {
            setHandleSaveParticipantsError(false);
            console.log("Saving participants: ", participants, " for conversation_id: ", conversation_id);

            // Update participants via API
            const response = await updateConversationParticipants(conversation_id, participants);
            console.log('updateConversationParticipants response: ', response);
            if(response.status === 200) {
                // Refresh the conversations list
                const refreshedConversations = await getConversations();
                setConversations(refreshedConversations);

                // Re-set the active conversation
                const activeConversation = refreshedConversations.find(
                    (conv) => conv.conversation_id === conversation_id
                );
                if (activeConversation) {
                    setActiveConversationId(activeConversation.conversation_id);
                    setSelectedConversation(activeConversation);
                }

                setParticipants("");
                console.log("Participants updated and active conversation refreshed.");
            }
        } catch (error) {
            console.error('Error updating participant', error);
            setHandleSaveParticipantsError(true);
        }
    };

    const handleNewMessage = async (message) => {
        if (message.conversation_id === conversation_id) {
            setMessages([...messages, message]);
            await setMessagesAsRead(conversation_id);
        }
        const refreshedConversations = await getConversations();
        setConversations(refreshedConversations);
    }

    // TODO Need to fix WebSockets in prod, constantly connecting and disconnecting and reconnecting etc
    //console.log("useMessagesWebSocket attempting to connect");
    //connectMessagesWebSocket(sessionUserId, handleNewMessage);

    return (
        <div className="conversation  h-full w-full flex flex-col p-8">
            {!participant_id &&(
                <div className="conversation__new-participants">
                    <p>
                        TO:
                    </p>
                    <input
                        type="text"
                        value={participants}
                        onChange={(e) => setParticipants(e.target.value)}
                        placeholder="Enter participant handle"
                    />
                    <button onClick={handleSaveParticipants}>Save</button>
                    {handleSaveParticipantsError && (
                        <div className="conversation__error">
                            <p>Invalid user handle</p>
                        </div>
                    )}
                </div>

            )}

            <div className="conversation__messages overflow-y-auto w-full h-full">
                <div className={``}>
                    {messages.length > 0 ? (
                        messages
                            .sort((a, b) => new Date(a.created_at) - new Date(b.created_at))
                            .map((message, index) => (
                                <Message
                                    key={index}
                                    session_user_id={sessionUserId}
                                    conversation_id={message.conversation_id}
                                    message_id={message.message_id}
                                    sender_id={message.sender_id}
                                    content={message.content}
                                    is_read={message.is_read}
                                    created_at={message.created_at}
                                />
                            ))
                    ) : (
                        <div>  </div>
                    )}
                </div>
            </div>
            <div className="conversation__message-form   bg-surface p-4 mt-4 rounded-2xl">
                <MessageForm
                    conversation_id={conversation_id}
                    participant_id={participant_id}
                    getMessages={getMessages}
                    setMessages={setMessages}
                    refreshMessages={refreshMessages}
                />
            </div>


        </div>
    )
}

export default Conversation;