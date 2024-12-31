import './MessageForm.css';
import { useState } from "react";
import useCreateMessage from '../../../api/hooks/useCreateMessage.js';

const MessageForm = () => {
    const [text, setText] = useState(`New Message`);
    const [formClicked, setFormClicked] = useState(false);
    const [expanded, setExpanded] = useState(false);

    const { createMessage, loading, error } = useCreateMessage();

    const styles = {
        textarea: {
            width: expanded ? '43%' : '43%',
            height: expanded ? '200%' : '100%',
        },
    };

    const handleSubmit = async () => {
        const message = {
            content: text,
        };
        console.log("Submitting post:", message);
        await createMessage(message);
        setText(""); // Reset the text field after submission
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
                    style={styles.textarea}
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