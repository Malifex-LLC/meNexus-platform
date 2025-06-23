import SynapseCard from '../SynapseCard/SynapseCard.jsx'
import {Link} from "react-router-dom";

const JoinedSynapsesTray = ({synapseMetadataList}) => {
    console.log('JoinedSynapsesTray synapseMetadataList: ', synapseMetadataList);
    return (
        <div className={'joinedSynapsesTray border border-border rounded-xl'}>
            {synapseMetadataList.length > 0 ? (
                synapseMetadataList
                    .map((synapse, index) => (
                        <Link
                            to={`/synapse/${synapse.identity.publicKey}`}
                            key={index}
                        >
                            <SynapseCard
                                key={index}
                                name={synapse.metadata.name}
                                description={synapse.metadata.description}
                                publicKey={synapse.identity.publicKey}
                            />
                        </Link>
                    ))
            ) : (
                <div>
                    No Joined Synapses found.
                </div>
            )}

        </div>
    );
}

export default JoinedSynapsesTray;