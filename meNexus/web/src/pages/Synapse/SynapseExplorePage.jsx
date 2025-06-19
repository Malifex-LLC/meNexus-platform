import SynapseExploreLayout from "../../layouts/SynapseLayout/SynapseExploreLayout.jsx";
import SynapseCard from "../../components/SynapseCard/SynapseCard.jsx";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import {useEffect, useState} from "react";
import {Link, useNavigate} from "react-router-dom";

const SynapseExplorePage = () => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUser } = useGetUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();

    const [user, setUser] = useState(null)
    const [synapses, setSynapses] = useState([]);
    const [synapseMetadataList, setSynapseMetadataList] = useState([]);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchUserData = async () => {
            try {
                const sessionUser = await getSessionUser();
                const userData = await getUser(sessionUser.data.publicKey);
                setUser(userData)
                setSynapses(userData.synapses);
            } catch (error) {
                console.error(error);
            }
        }
        fetchUserData();
    },[]);

    useEffect(() => {
        const fetchAllMetadata = async () => {
            const results = await Promise.all(
                synapses.map(async (synapse) => {
                    return await getSynapseMetadata(synapse);
                })
            );
            setSynapseMetadataList(results);
        };

        if (synapses.length > 0) {
            fetchAllMetadata();
        }
    }, [synapses]);

    if (!user) {
        return <div>Loading User Data...</div>;
    }

    return (
        <SynapseExploreLayout>
            <div className={'p-4 text-7xl text-brand'}>
                Joined Synapses
            </div>
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
        </SynapseExploreLayout>
    );
};

export default SynapseExplorePage;