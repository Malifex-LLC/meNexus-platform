import SynapseMembersPanel from "../SynapseMembersPanel/SynapseMembersPanel.jsx";
import SynapseActivityPanel from "../SynapseActivityPanel/SynapseActivityPanel.jsx";
import {useSwipeable} from "react-swipeable";

const SynapseInfoTray = ({activeSidebarTab, setActiveSidebarTab, members, isLocalSynapse, synapseMetadata, user}) => {
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActiveSidebarTab('activity'),
        onSwipedRight: () => setActiveSidebarTab('members'),
    });
    return (
        <div className="flex flex-col h-full bg-surface/70">
            {/* Tabs */}
            <div className="flex justify-around p-4 gap-4 bg-surface border-b border-border text-2xl text-foreground font-montserrat">
                <button
                    onClick={() => setActiveSidebarTab('members')}
                    className={`${activeSidebarTab === 'members' ? 'text-brand font-bold' : 'hover:text-brand/50 hover:cursor-pointer'}`}
                >
                    Members
                </button>
                <button
                    onClick={() => setActiveSidebarTab('activity')}
                    className={`${activeSidebarTab === 'activity' ? 'text-brand font-bold' : 'hover:text-brand/50 hover:cursor-pointer'}`}
                >
                    Activity
                </button>
            </div>

            {/* Content */}
            <div className="flex-1 min-h-0 overflow-y-auto"
                 {...swipeHandlers}
            >
                {activeSidebarTab === 'members' ? (
                    <SynapseMembersPanel members={members} />
                ) : (
                    <SynapseActivityPanel
                        isLocalSynapse={isLocalSynapse}
                        synapseMetadata={synapseMetadata}
                        publicKey={user.publicKey}
                        user={user}
                    />
                )}
            </div>
        </div>
    )
}

export default SynapseInfoTray