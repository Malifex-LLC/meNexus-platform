// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import PostBoard from '../PostBoard/PostBoard.jsx';
import {useState} from "react";

const PostBoardsPanel = ({boards, activeBoard, setActiveBoard}) => {


    return (
        <div className={''}>
            <div className="text-center rounded-xl border-b border-border">
                Boards
            </div>

            {boards.map((board) => (
                <button                       /* button = semantic + focus support  */
                    key={board}
                    onClick={() => setActiveBoard(board)}
                    className={`
            rounded-xl p-2 mt-4 w-full text-left text-xl shadow-lg
            hover:bg-surface hover:translate-y-[-2px]
            ${activeBoard === board ? "bg-primary text-background" : "bg-background"}
          `}
                >
                    <PostBoard channel={board} />
                </button>
            ))}
        </div>
    );
};


export default PostBoardsPanel;