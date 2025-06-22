import ChatMessageForm from "../ChatMessageForm/ChatMessageForm.jsx";
import ChannelsPanel from "../Channels/ChannelsPanel/ChannelsPanel.jsx";
import ChatWindow from "../ChatWindow/ChatWindow.jsx";

const ChatPanel = () => {
    return (
        <div className={'flex flex-col w-full h-full text-foreground'}>
            <div className={`flex flex-row h-1/2 w-full p-4 gap-4`}>
                <div className={`w-1/5 overflow-y-auto h-full p-4 border border-border`}>
                    <ChannelsPanel/>
                </div>
                <div className={`flex-1 overflow-y-auto w-full border border-border`}>
                    <ChatWindow/>
                </div>
            </div>
            <div className={'p-4'}>
                <ChatMessageForm/>
            </div>
        </div>
    );
}

export default ChatPanel;