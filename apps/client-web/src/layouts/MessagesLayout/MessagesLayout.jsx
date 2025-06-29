// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from '../../components/Header/Header.jsx'
import Navigation from "../../components/Navigation/Navigation.jsx";
import {useEffect, useState} from "react";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import {useNavigate} from "react-router-dom";

const MessagesLayout = ({children}) => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUser } = useGetUser();
    const [sessionUser, setSessionUser ] = useState(null)

    const [user, setUser] = useState(null)
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

    if (!user) {
        return (
            <div className={'h-screen bg-background text-foreground text-center p-4'}>Loading...</div>
        )
    }
    return (
        <div className="messages-layout flex flex-col md:flex-row h-screen  pt-17 bg-background  ">
            <Header
            user={user}
            />
            <div className="messages-layout__conversation-menu mb-8  ">
                {children[0] }
            </div>
            <div className="messages-layout__conversation-content flex-1 overflow-hidden md:mr-16 lg:mr-32">
                {children[1]}
            </div>
        </div>
    );
}

export default MessagesLayout;