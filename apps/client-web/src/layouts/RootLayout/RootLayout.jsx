import {useEffect, useState} from "react";
import {Outlet, useNavigate} from "react-router-dom";
import Header from "../../components/Header/Header.jsx";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";

const RootLayout = () => {
    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const [localSynapseMetadata, setLocalSynapseMetadata] = useState(null);
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const navigate = useNavigate(); // React Router navigate

    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                console.log("Fetching current user session...");
                const response = await getSessionUser();
                setSessionUser(response.data)
            } catch (error) {
                console.error("Error fetching current session user:", error);
                navigate('/login');
            }
        }
        fetchSessionUser();
    }, [])

    useEffect(() => {
        if(!sessionUser || !sessionUser.publicKey) return;
        const fetchUser = async () => {
            try {
                const response = await getUser(sessionUser.publicKey);
                setUser(response);
            } catch (error) {
                console.error("Error fetching current user:", error);
            }
        }
        fetchUser();
    }, [sessionUser])

    useEffect(() => {
        const getMetadata = async () => {
            try {
                const metadata = await getSynapseMetadata();
                setLocalSynapseMetadata(metadata);
            } catch (error) {
                console.error(error);
            }
        }
        getMetadata()
    }, [])

    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    return (
        <div className="h-[100dvh] flex flex-col ">
            <Header user={user} localSynapseMetadata={localSynapseMetadata} />
            <main className="">
                <Outlet context={{ sessionUser, user, localSynapseMetadata }} />
            </main>
        </div>
    );
}

export default RootLayout