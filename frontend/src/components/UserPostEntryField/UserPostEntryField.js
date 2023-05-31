import React, {useState} from "react";

const UserPostEntryField = () => {
    const [expanded, setExpanded] = useState(false);
    const [text, setText] = useState("");
    const styles = {
        textarea: {
            width: expanded ? '43%' : '43%',
            height: expanded ? '200%' : '100%',
        },
    };


    return (
        <div onClick={() => setExpanded(!expanded)}>
      <textarea
          style={styles.textarea}
          value={text}
          onChange={(e) => setText(e.target.value)}
      />
        </div>
    );
}

export default UserPostEntryField;