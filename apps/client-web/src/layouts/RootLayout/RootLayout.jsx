import {useEffect, useState} from "react";
import {Outlet, useNavigate} from "react-router-dom";
import Header from "../../components/Header/Header.jsx";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import {setAccessToken} from "../../utils/authUtils.js";

const RootLayout = () => {
    const [booted, setBooted] = useState(false);
    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const [localSynapseMetadata, setLocalSynapseMetadata] = useState(null);
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const navigate = useNavigate(); // React Router navigate

    useEffect(() => {
        (async () => {
            try {
                // 1) silent refresh to repopulate in-memory access token
                const r = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/auth/refresh`, { method: 'POST', credentials: 'include' });
                if (r.ok) {
                    const j = await r.json();
                    setAccessToken(j.accessToken);
                }
                // 2) now hit protected endpoint
                const response = await getSessionUser();
                setSessionUser(response.data.user);
                setUser(response.data.user)
            } catch (error) {
                // don’t navigate until boot is done (prevents flicker loops)
                console.error(error);
                navigate('/login');
            } finally {
                setBooted(true);
            }
        })();
    }, []);

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