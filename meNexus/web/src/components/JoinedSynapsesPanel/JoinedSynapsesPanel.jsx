import SynapseCard from "../SynapseCard/SynapseCard.jsx";
import {useEffect, useState} from "react";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import {Link} from "react-router-dom";

const JoinedSynapsesPanel = ({synapses}) => {
    const [synapseMetadataList, setSynapseMetadataList] = useState([]);
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { getSynapseMetadata } = useGetSynapseMetadata();


    useEffect(() => {
        const fetchAllMetadata = async () => {
            const localSynapseMetadata = await getSynapseMetadata();
            const results = await Promise.all(
                synapses.map(async (synapse) => {
                    if (synapse === localSynapseMetadata.identity.publicKey) {
                        return localSynapseMetadata;
                    }
                    return await fetchRemoteSynapseMetadata(synapse);
                })
            );
            setSynapseMetadataList(results);
        };

        if (synapses.length > 0) {
            fetchAllMetadata();
        }
    }, [synapses]);
    return (
        <div className={' bg-background text-foreground p-4'}>
            <div className={'text-3xl text-center'}>
                Joined Synapses
            </div>
            <div>
                {synapseMetadataList.map((metadata, index) => (
                    <Link to={`/synapse/${metadata.identity.publicKey}`} key={index}>
                        <SynapseCard
                            key={index}
                            name={metadata.metadata.name}
                            description={metadata.metadata.description}
                            publicKey={metadata.identity.publicKey}
                        />
                    </Link>
                ))}
            </div>
        </div>
    );
}

export default JoinedSynapsesPanel;