import SynapseControlBar from "./SynapseControlBar.jsx";
import { CgFeed } from "react-icons/cg";
import { FaUsersViewfinder } from "react-icons/fa6";
import PostBoardsPanel from "../Posting/PostBoardsPanel/PostBoardsPanel.jsx";
import ChattingChannelsPanel from "../Chatting/ChattingChannelsPanel/ChattingChannelsPanel.jsx";

const SynapseControlBarTray = ({user, activeMiddlebarTab, setActiveMiddlebarTab, boards, activeBoard, setActiveBoard, channels, activeChannel, setActiveChannel}) => {
    return (
        <div className="flex flex-col h-full w-full bg-surface/70">
            <SynapseControlBar synapses={user.synapses} publicKey={user.publicKey} />

            {/* Feed/Chat selector (your original switcher) */}
            <div className="p-2 mx-4 my-2 border border-border rounded-xl flex justify-around bg-surface text-4xl text-foreground shadow-2xl">
                <button
                    className={`flex justify-center w-full h-full border-r border-border ${activeMiddlebarTab === 'feed' ? 'text-brand font-bold' : 'hover:text-brand/50 hover:cursor-pointer active:scale-90'}`}
                    onClick={() => setActiveMiddlebarTab('feed')}
                    aria-label="Show Feed Boards"
                >
                    <CgFeed />
                </button>
                <button
                    className={`flex justify-center w-full h-full ${activeMiddlebarTab === 'chat' ? 'text-brand font-bold' : 'hover:text-brand/50 hover:cursor-pointer active:scale-90'}`}
                    onClick={() => setActiveMiddlebarTab('chat')}
                    aria-label="Show Chat Channels"
                >
                    <FaUsersViewfinder />
                </button>
            </div>

            {/* Boards / Channels */}
            <div className="flex-1 min-h-0 overflow-y-auto px-2 pb-2">
                {activeMiddlebarTab === 'feed' ? (
                    <div className="flex w-full rounded-xl shadow-2xl">
                        <PostBoardsPanel
                            boards={boards}
                            activeBoard={activeBoard}
                            setActiveBoard={setActiveBoard}
                        />
                    </div>
                ) : (
                    <div className="flex w-full rounded-xl shadow-2xl">
                        <ChattingChannelsPanel
                            channels={channels}
                            activeChannel={activeChannel}
                            setActiveChannel={setActiveChannel}
                        />
                    </div>
                )}
            </div>
        </div>
    )
}

export default SynapseControlBarTray