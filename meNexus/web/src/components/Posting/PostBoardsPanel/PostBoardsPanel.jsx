// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import PostBoard from '../PostBoard/PostBoard.jsx';
import {useState} from "react";

const PostBoardsPanel = () => {
    const boards = ["Main Chat", "Memes", "Music Share", "News PostBoard", "Gaming"];
    const [activeChannel, setActiveChannel] = useState("Main Chat");

    return (
        <div className={''}>
            <div className="text-center rounded-xl border-b border-border">
                Boards
            </div>

            {boards.map((channel) => (
                <button                       /* button = semantic + focus support  */
                    key={channel}
                    onClick={() => setActiveChannel(channel)}
                    className={`
            rounded-xl p-2 mt-4 w-full text-left text-xl shadow-lg
            hover:bg-surface hover:translate-y-[-2px]
            ${activeChannel === channel ? "bg-primary text-background" : "bg-background"}
          `}
                >
                    <PostBoard channel={channel} />
                </button>
            ))}
        </div>
    );
};


export default PostBoardsPanel;