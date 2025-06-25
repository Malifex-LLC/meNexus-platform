import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";

const UserJoinedSynapsesPanel = ({user}) => {
    return (
        <div>
            <JoinedSynapsesPanel synapses={user.synapses} />
        </div>
    );
}

export default UserJoinedSynapsesPanel;