import { useState } from "react";
import useCreateMessage from '../../../api/hooks/useCreateMessage.js';
import { IoSend } from "react-icons/io5";


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
        <div className="message-form flex gap-4 bg-surface px-8">
            <div className={`w-full`} onClick={handleFormClick}>
                <textarea
                    className="message-form__entry-field w-full h-full bg-zinc-700 text-foreground rounded-2xl"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                />
            </div>
            <button className="message-form__button text-2xl text-foreground " onClick={handleSubmit} disabled={loading}>
                {loading ? "Sending..." : <IoSend />}
            </button>
            {error && <div className="error">Error: {error}</div>}
        </div>
    );
}

export default MessageForm;