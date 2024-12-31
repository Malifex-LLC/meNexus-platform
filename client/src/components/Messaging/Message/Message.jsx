import './Message.css';

const Message = ({
                    message_id,
                    date,
                    content,
                    onEdit,
                    onDelete,
                    isEditing,
                    editedContent,
                    onContentChange,
                    onSave,
                 }) => {
    const isOwner = user_id === session_user_id;

    return (
        <div className={`message ${isEditing ? "message--editing" : ""}`}>
            <p>{content}</p>
        </div>
    )
}

export default Message;