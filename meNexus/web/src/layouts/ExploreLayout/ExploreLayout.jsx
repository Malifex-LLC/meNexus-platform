import Header from "../../components/Header/Header.jsx";
import {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate} from "react-router-dom";

const ExploreLayout = ({children}) => {
    const navigate = useNavigate(); // React Router navigate

    const [sessionUser, setSessionUser ] = useState(null)
    const [user, setUser] = useState(null)
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();


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

    if (!user || !user.publicKey) {
        return <>Loading...</>;
    }
    return (
        <div className={'h-screen pt-17 bg-background'}>
            <Header
                user={user}
            />
            {children}
        </div>
    );
}

export default ExploreLayout;