// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import PostBoard from '../PostBoard/PostBoard.jsx';
import {useState} from "react";

const PostBoardsPanel = ({boards, activeBoard, setActiveBoard}) => {


    return (
        <div className={'flex flex-col p-4 w-full'}>
            <div className="text-foreground text-center text-xl border-b border-border font-montserrat">
                Boards
            </div>

            {boards.map((board) => (
                <button
                    key={board}
                    onClick={() => setActiveBoard(board)}
                    className={` p-2 mt-4 w-full text-left text-xl text-foreground shadow-lg 
                                 border border-border rounded-xl hover:bg-brand/10 hover:translate-y-[-2px] active:scale-90
            ${activeBoard === board ? "bg-brand/60 text-background" : "bg-surface"}
          `}
                >
                    <PostBoard channel={board} />
                </button>
            ))}
        </div>
    );
};


export default PostBoardsPanel;