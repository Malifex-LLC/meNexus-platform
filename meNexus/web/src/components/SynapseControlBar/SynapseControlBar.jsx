import {Link, useLocation, useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import { IoMdArrowDropdown } from "react-icons/io";
import JoinedSynapsesTray from "../JoinedSynapsesTray/JoinedSynapsesTray.jsx";

const SynapseControlBar = ({synapses = []}) => {
    const location = useLocation();
    const isActive = (path) => location.pathname.startsWith(path) ? "text-brand" : "text-foreground";
    const {synapsePublicKey} = useParams();
    const [synapseMetadataList, setSynapseMetadataList] = useState([]);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [showJoinedSynapsesTray, setShowJoinedSynapsesTray] = useState(false);
    const { fetchRemoteSynapseMetadata } = useFetchRemoteSynapseMetadata();
    const { getSynapseMetadata } = useGetSynapseMetadata();

    useEffect(() => {
        const fetchAllMetadata = async () => {
            const localSynapseMetadata = await getSynapseMetadata();
            const results = await Promise.all(
                synapses.map(async (synapse) => {
                    if (synapse === localSynapseMetadata.identity.publicKey) {
                        if (localSynapseMetadata.identity.publicKey === synapsePublicKey) {
                            setSynapseMetadata(localSynapseMetadata);
                        }
                        return localSynapseMetadata;
                    }
                    const fetchedSynapseMetadata = await fetchRemoteSynapseMetadata(synapse);
                    if(fetchedSynapseMetadata.identity.publicKey === synapsePublicKey) {
                        setSynapseMetadata(fetchedSynapseMetadata);
                        return fetchedSynapseMetadata;
                    }
                    return fetchedSynapseMetadata;
                })
            );
            setSynapseMetadataList(results);
        };

        if (synapses.length > 0) {
            fetchAllMetadata();
        }
    }, [synapsePublicKey, synapses]);

    const toggleJoinedSynapsesTray = () => {
        setShowJoinedSynapsesTray((prevState) => !prevState);
    }

    return (
        <div className=" p-4 mt-10  gap-6 bg-background text-foreground ">
            <div>
                {synapseMetadata && synapseMetadataList ? (
                    <div className="flex flex-row pt-4 text-foreground">
                        <div>
                            <h1 className="text-3xl text-brand font-bold">{synapseMetadata.metadata.name}</h1>
                            <p className="text-lg text-foreground">
                                {synapseMetadata.metadata.description}
                            </p>
                            <p className="text-xs ">
                                publicKey: {synapseMetadata.identity.publicKey}
                            </p>
                        </div>
                        <div className={'flex flex-col'}>
                            <div
                                className={'flex text-3xl'}
                                onClick={toggleJoinedSynapsesTray}
                            >
                                <div className={'flex h-full text-5xl cursor-pointer mt-10'}>
                                    <IoMdArrowDropdown />
                                </div>
                            </div>
                            <div>
                                {showJoinedSynapsesTray && (
                                    <div className="absolute  z-50 bg-header-bg/70 rounded-2xl shadow-2xl w-2xl">
                                        <JoinedSynapsesTray
                                            synapseMetadataList={synapseMetadataList}
                                        />
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                ) : (
                    <div className="p-4">Loading synapse info…</div>
                )}
            </div>
        </div>
    )
}

export default SynapseControlBar;