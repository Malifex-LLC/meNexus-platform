// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate, useParams} from "react-router-dom";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import {useSwipeable} from "react-swipeable";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import SynapseUsersPanel from "../../components/SynapseUsersPanel/SynapseUsersPanel.jsx";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import ChatPanel from "../../components/Chatting/ChatPanel/ChatPanel.jsx";
import PostsPanel from "../../components/Posting/PostingPanel/PostsPanel.jsx";
import useFetchRemotePosts from "../../api/hooks/useFetchRemotePosts.js";
import SynapseControlBar from "../../components/SynapseControlBar/SynapseControlBar.jsx";
import JoinedSynapsesPanel from "../../components/JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useGetAllPosts from "../../api/hooks/useGetAllPosts.js";
import { CgFeed } from "react-icons/cg";
import { FaUsersViewfinder } from "react-icons/fa6";
import useGetBoardPosts from "../../api/hooks/useGetBoardPosts.js";
import useFetchRemoteBoardPosts from "../../api/hooks/useFetchRemoteBoardPosts.js";
import useGetSynapsePostBoards from "../../api/hooks/useGetSynapsePostBoards.js";
import useFetchRemoteSynapsePostBoards from "../../api/hooks/useFetchRemoteSynapsePostBoards.js";
import useGetChannelChatMessages from "../../api/hooks/useGetChannelChatMessages.js";
import useGetSynapseChatChannels from "../../api/hooks/useGetSynapseChatChannels.js";

const SynapseLayout =({ children }) => {

    const { synapsePublicKey } = useParams(); // Extract synapsePublicKey from the URL (if available)
    const [isLocalSynapse, setIsLocalSynapse] = useState(true);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [sessionUser, setSessionUser ] = useState({})
    const [user, setUser] = useState({})
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { fetchRemoteSynapseMetadata, loading, error } = useFetchRemoteSynapseMetadata();
    const navigate = useNavigate(); // React Router navigate
    const [boards, setBoards] = useState(null);
    const [activeBoard, setActiveBoard] = useState(null);
    const [posts, setPosts] = useState([]); // State for synapse posts
    const [channels, setChannels] = useState([]);
    const [activeChannel, setActiveChannel] = useState(null);
    const [chatMessages, setChatMessages] = useState([]);

    const [activeSidebarTab, setActiveSidebarTab] = useState("activity"); // or "chat"
    const [activeMiddlebarTab, setActiveMiddlebarTab] = useState("feed");


    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { getSynapsePostBoards } = useGetSynapsePostBoards();
    const { getBoardPosts } = useGetBoardPosts();
    const { getSynapseChatChannels } = useGetSynapseChatChannels();
    const { getChannelChatMessages } = useGetChannelChatMessages();
    const { fetchRemoteSynapsePostBoards } = useFetchRemoteSynapsePostBoards();
    const { fetchRemoteBoardPosts, loading: remoteBoardPostsLoading, error: remoteBoardPostsError } = useFetchRemoteBoardPosts();

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
            if (!sessionUser.publicKey) {
                return;
            }
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
        const fetchSynapseMetadata = async () => {
            try {
                const localSynapseData = await getSynapseMetadata();
                if (localSynapseData.identity.publicKey === synapsePublicKey) {
                    setSynapseMetadata(localSynapseData);
                    setIsLocalSynapse(true);
                    const synapseBoards = await getSynapsePostBoards();
                    setBoards(synapseBoards)
                    setActiveBoard(synapseBoards[0])
                    const synapseChannels = await getSynapseChatChannels();
                    setChannels(synapseChannels);
                    setActiveChannel(synapseChannels[0])
                } else {
                    try {
                        const synapseMetadataResponse = await fetchRemoteSynapseMetadata(synapsePublicKey);
                        setSynapseMetadata(synapseMetadataResponse);
                        setIsLocalSynapse(false);
                        const synapseBoards = await fetchRemoteSynapsePostBoards(synapsePublicKey);
                        setBoards(synapseBoards);
                        setActiveBoard(synapseBoards[0])
                    } catch (error) {
                        console.error("Error fetching remote Synapse data: ", error);
                    }
                }
            } catch (error) {
                console.error("Error fetching local Synapse data:", error);
            }
        };
        fetchSynapseMetadata();
    },[synapsePublicKey]);

    useEffect(() => {
        const fetchSynapsePosts = async () => {
            if (!synapsePublicKey) {
                return;
            }
            if (isLocalSynapse) {
                try {
                    const boardPosts = await getBoardPosts(activeBoard);
                    setPosts(boardPosts);

                } catch (error) {
                    console.error("Error fetching Synapse posts: ", error);
                }
            } else {
                try {
                    const remoteBoardPosts = await fetchRemoteBoardPosts(synapsePublicKey, activeBoard);
                    setPosts(remoteBoardPosts);
                } catch (error) {
                    console.error("Error fetching remote Synapse posts: ", error);
                }
            }

        };
        fetchSynapsePosts();
    },[synapsePublicKey, isLocalSynapse, synapseMetadata, activeBoard]);

    useEffect(() => {
        const fetchSynapseChatMessages = async () => {
            if (!synapsePublicKey) {
                return
            }
            if (isLocalSynapse) {
                try {
                    const channelChats = await getChannelChatMessages(activeChannel);
                    setChatMessages(channelChats);
                } catch (error) {
                    console.error("Error fetching Synapse chat messages: ", error);
                }
            } else {
                try {
                    console.log('Fetching remote synapse chat messages...')
                } catch (error) {
                    console.error("Error fetching remote Synapse chat messages :", error);
                }
            }
        }
        fetchSynapseChatMessages();
    }, [synapsePublicKey, isLocalSynapse, synapseMetadata, activeChannel])


    if (!user || !user.publicKey) {
        return <div className={'bg-background text-foreground'}>Loading Synapse...</div>;
    }

    if (!boards) {
        return <div className={'bg-background text-foreground'}>Loading Synapse boards...</div>
    }


    return (
        <div className="home-layout h-screen flex flex-col ">
            {/* Remove the bg-background above to see the magic below */}
            {/*<div className={`absolute top-0 left-o -z-1 pt-17 h-screen w-screen bg-gradient-to-b from-background via-primary to-background backdrop-blur-lg `}/>*/}

            {/* Header */}
            <div className="sticky top-0 h-17 z-50 border-b border-border">
                <Header
                    user={user}
                />
            </div>

            {/* Main Grid */}
            <div className="flex-1 min-h-0 w-full lg:grid lg:grid-cols-12 overflow-hidden ">

                {/* Main Content (Center Column) */}
                <div className="flex flex-col flex-1 p-4 min-h-0 w-full lg:col-span-9 overflow-hidden">

                    {/* Synapse Control Bar */}
                    <div className=" mx-4 shadow-2xl border border-border rounded-xl">
                        <SynapseControlBar
                            synapses={user.synapses}
                            publicKey={user.publicKey}
                        />
                    </div>

                    {/* Scrollable PostingPanel */}
                    <div className={'hidden lg:flex flex-col flex-1 min-h-0  lg:col-span-9 '}>

                        {/* Tab Switcher */}
                        <div className={'p-2 mx-4 my-2 border border-border rounded-xl flex justify-around  bg-background  p-2 text-4xl ' +
                            'text-foreground '}>
                            <button
                                className={`flex justify-center w-full h-full gap-2 border-r border-border hover:cursor-pointer
                                ${activeMiddlebarTab === "feed" ? "text-brand font-bold" : "hover:text-brand"}`}
                                onClick={() => setActiveMiddlebarTab("feed")}
                            >
                                <CgFeed />
                            </button>
                            <button
                                className={`flex justify-center w-full h-full gap-2 hover:cursor-pointer
                                ${activeMiddlebarTab === "chat" ? "text-brand font-bold" : "hover:text-brand"}`}
                                onClick={() => setActiveMiddlebarTab("chat")}
                            >
                                <FaUsersViewfinder />
                            </button>
                        </div>

                        {/* Content */}
                        <div className={'flex flex-1 min-h-0 overflow-y-auto '}>
                            {activeMiddlebarTab === "feed" ? (
                                <div className={'mx-4 my-2  rounded-xl shadow-2xl'}>
                                    <PostsPanel
                                        isLocalSynapse={isLocalSynapse}
                                        publicKey={sessionUser.publicKey}
                                        synapsePublicKey={synapsePublicKey}
                                        boards={boards}
                                        activeBoard={activeBoard}
                                        setActiveBoard={setActiveBoard}
                                        posts={posts}
                                        setPosts={setPosts}
                                    />
                                </div>
                            ) : activeMiddlebarTab === "chat" ? (
                                <div className={'flex w-full h-full shadow-2xl rounded-xl'}>
                                    <ChatPanel
                                        isLocalSynapse={isLocalSynapse}
                                        publicKey={sessionUser.publicKey}
                                        synapsePublicKey={synapsePublicKey}
                                        channels={channels}
                                        activeChannel={activeChannel}
                                        setActiveChannel={setActiveChannel}
                                        chatMessages={chatMessages}
                                        setChatMessages={setChatMessages}
                                    />
                                </div>

                            ) : (
                                <div className="flex flex-col  w-full lg:col-span-6 ">
                                    unknown tab content
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* Right Column (Activity Feed + Users) */}
                <div className="hidden lg:flex flex-col m-4 rounded-xl lg:col-span-3 overflow-hidden bg-background">
                    {/* Tab Switcher */}
                    <div className="flex justify-around border border-border bg-background
                    p-2 m-4 text-xl text-foreground shadows-2xl rounded-xl">
                        <button
                            onClick={() => setActiveSidebarTab("activity")}
                            className={`${activeSidebarTab === "activity" ? "text-brand font-bold" : "hover:text-brand"}`}
                        >
                            Members
                        </button>
                        <button
                            onClick={() => setActiveSidebarTab("chat")}
                            className={`${activeSidebarTab === "chat" ? "text-brand font-bold" : "hover:text-brand"}`}
                        >
                            Activity
                        </button>
                    </div>

                    {/* Content */}
                    <div className=" overflow-y-auto p-4 m-4  bg-background border border-border rounded-xl shadow-2xl ">
                        {activeSidebarTab === "activity" ? (
                            <>
                                <SynapseUsersPanel />
                            </>
                        ) : (
                            <ActivityFeed />
                        )}
                    </div>
                </div>

            </div>
        </div>
    );


};

export default SynapseLayout;