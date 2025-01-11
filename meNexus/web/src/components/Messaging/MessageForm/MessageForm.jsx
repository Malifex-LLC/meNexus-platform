import './MessageForm.css';
import { useState } from "react";
import useCreateMessage from '../../../hooks/api/useCreateMessage.js';

const MessageForm = ({
                         conversation_id,
                         participant_id,
                         getMessages,
                         setMessages,
                         refreshMessages
}) => {
    const [text, setText] = useState(`New Message`);
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createMessage, loading, error } = useCreateMessage(() => refreshMessages());

    const handleSubmit = async () => {
        if (!text.trim()) return; // Avoid sending empty messages
        const message = {
            participant_id: participant_id,
            content: text,
        };
        console.log("Submitting message:", message);
        await createMessage(conversation_id, message);
        setText(""); // Reset the text field after submission
        await refreshMessages(getMessages, conversation_id, setMessages); // Refresh messages
    };

    const handleFormClick = () => {
        if (!formClicked && text === `New Message`) {
            setText("");
        }
        setFormClicked(true);
    };

    return (
        <div className="message-form">
            <div onClick={handleFormClick}>
                <textarea
                    className="message-form__entry-field"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="message-form__button" onClick={handleSubmit} disabled={loading}>
                {loading ? "Sending..." : "Send"}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
}

export default MessageForm;