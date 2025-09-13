// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Header from "../../components/Header/Header.jsx";
import React, {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate, useParams} from "react-router-dom";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import {useSwipeable} from "react-swipeable";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
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
import useFetchRemoteSynapseChatChannels from "../../api/hooks/useFetchRemoteSynapseChatChannels.js";
import useFetchRemoteChannelChats from "../../api/hooks/useFetchRemoteChannelChats.js";
import SynapseMembersPanel from "../../components/SynapseMembersPanel/SynapseMembersPanel.jsx";
import useGetSynapseMembers from "../../api/hooks/useGetSynapseMembers.js";
import useFetchRemoteSynapseMembers from "../../api/hooks/useFetchRemoteSynapseMembers.js";
import UserActivityPanel from "../../components/UserActivityPanel/UserActivityPanel.jsx";
import SynapseActivityPanel from "../../components/SynapseActivityPanel/SynapseActivityPanel.jsx";
import PostBoardsPanel from "../../components/Posting/PostBoardsPanel/PostBoardsPanel.jsx";
import ChattingChannelsPanel from "../../components/Chatting/ChattingChannelsPanel/ChattingChannelsPanel.jsx";
import { TbActivityHeartbeat } from "react-icons/tb";
import { FiActivity } from "react-icons/fi";


const SynapseLayout =({ children }) => {

    const { synapsePublicKey } = useParams(); // Extract synapsePublicKey from the URL (if available)
    const [isLocalSynapse, setIsLocalSynapse] = useState(true);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [sessionUser, setSessionUser ] = useState({})
    const [user, setUser] = useState({})
    const { getUser } = useGetUser();
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const navigate = useNavigate(); // React Router navigate
    const [members, setMembers] = useState(null);
    const [boards, setBoards] = useState(null);
    const [activeBoard, setActiveBoard] = useState(null);
    const [posts, setPosts] = useState([]); // State for synapse posts
    const [channels, setChannels] = useState(null);
    const [activeChannel, setActiveChannel] = useState(null);
    const [chatMessages, setChatMessages] = useState([]);

    const [activeSidebarTab, setActiveSidebarTab] = useState("activity");
    const [activeMiddlebarTab, setActiveMiddlebarTab] = useState("feed");


    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { getSynapseMembers } = useGetSynapseMembers();
    const { getSynapsePostBoards } = useGetSynapsePostBoards();
    const { getBoardPosts } = useGetBoardPosts();
    const { getSynapseChatChannels } = useGetSynapseChatChannels();
    const { getChannelChatMessages } = useGetChannelChatMessages();
    const { fetchRemoteSynapseMetadata, loading, error } = useFetchRemoteSynapseMetadata();
    const { fetchRemoteSynapseMembers } = useFetchRemoteSynapseMembers();
    const { fetchRemoteSynapsePostBoards } = useFetchRemoteSynapsePostBoards();
    const { fetchRemoteBoardPosts, loading: remoteBoardPostsLoading, error: remoteBoardPostsError } = useFetchRemoteBoardPosts();
    const { fetchRemoteSynapseChatChannels } = useFetchRemoteSynapseChatChannels();
    const { fetchRemoteChannelChats } = useFetchRemoteChannelChats();

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
                    const synapseMembers = await getSynapseMembers();
                    setMembers(synapseMembers);
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
                        const synapseMembers = await fetchRemoteSynapseMembers(synapsePublicKey);
                        setMembers(synapseMembers);
                        const synapseBoards = await fetchRemoteSynapsePostBoards(synapsePublicKey);
                        setBoards(synapseBoards);
                        setActiveBoard(synapseBoards[0])
                        const synapseChannels = await fetchRemoteSynapseChatChannels(synapsePublicKey);
                        setChannels(synapseChannels);
                        setActiveChannel(synapseChannels[0])
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
                    const channelChats = await fetchRemoteChannelChats(synapsePublicKey, activeChannel);
                    setChatMessages(channelChats);
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

    if (!channels) {
        return <div className={'bg-background text-foreground'}>Loading Synapse channels...</div>
    }


    return (
        <div className="flex flex-col h-screen">
            {/* Remove the bg-background above to see the magic below */}
            {/*<div className={`absolute top-0 left-o -z-1 pt-17 h-screen w-screen bg-gradient-to-b from-background via-primary to-background backdrop-blur-lg `}/>*/}

            {/* Header */}
            <div className="sticky top-0 h-17 z-50 border-b border-border">
                <Header
                    user={user}
                />
            </div>

            {/* Main Grid */}
            <div className="flex flex-col  p-4 gap-4 h-full min-h-0 w-full lg:grid lg:grid-cols-12 overflow-hidden">
                {/* Synapse Control Bar */}
                <div className="flex flex-col shadow-2xl border border-border rounded-xl lg:col-span-3 bg-surface/70">
                    <SynapseControlBar
                        synapses={user.synapses}
                        publicKey={user.publicKey}
                    />
                    {/* Tab Switcher */}
                    <div className={'p-2 mx-4 my-2 border border-border rounded-xl flex justify-around  bg-surface  p-2 text-4xl ' +
                        'text-foreground '}>
                        <button
                            className={`flex justify-center w-full h-full gap-2 border-r border-border hover:cursor-pointer
                                ${activeMiddlebarTab === "feed" ? "text-brand font-bold" : "hover:text-brand/50"}`}
                            onClick={() => setActiveMiddlebarTab("feed")}
                        >
                            <CgFeed />
                        </button>
                        <button
                            className={`flex justify-center w-full h-full gap-2 hover:cursor-pointer
                                ${activeMiddlebarTab === "chat" ? "text-brand font-bold" : "hover:text-brand/50"}`}
                            onClick={() => setActiveMiddlebarTab("chat")}
                        >
                            <FaUsersViewfinder />
                        </button>
                    </div>

                    <div>
                        {activeMiddlebarTab === "feed" ? (
                            <div className={'flex w-full rounded-xl shadow-2xl'}>
                                <PostBoardsPanel
                                    boards={boards}
                                    activeBoard={activeBoard}
                                    setActiveBoard={setActiveBoard}
                                />
                            </div>
                        ) : activeMiddlebarTab === "chat" ? (
                            <div className={'flex w-full rounded-xl shadow-2xl'}>
                                <ChattingChannelsPanel
                                    channels={channels}
                                    activeChannel={activeChannel}
                                    setActiveChannel={setActiveChannel}
                                />
                            </div>

                        ) : (
                            <div className="flex flex-col  w-full lg:col-span-6 ">
                                unknown tab content
                            </div>
                        )}
                    </div>
                </div>

                {/* Main Content (Center Column) */}
                <div className="flex flex-col flex-1  min-h-0 w-full lg:col-span-6 overflow-hidden">
                    {/* Scrollable PostingPanel */}
                    <div className={'hidden lg:flex flex-col flex-1 min-h-0  lg:col-span-6 '}>
                        {/* Content */}
                        <div className={'flex flex-1 min-h-0 overflow-y-auto '}>
                            {activeMiddlebarTab === "feed" ? (
                                <div className={' w-full rounded-xl shadow-2xl'}>
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
                                        key={synapseMetadata?.identity?.publicKey}
                                        synapseMetadata={synapseMetadata}
                                        publicKey={sessionUser.publicKey}
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
                <div className="hidden lg:flex flex-col w-full rounded-xl lg:col-span-3 overflow-hidden bg-surface/70  border border-border">
                    {/* Tab Switcher */}
                    <div className="flex justify-around p-4 gap-4 bg-surface border-b border-border text-2xl text-foreground shadows-2xl ">
                        <button
                            onClick={() => setActiveSidebarTab("members")}
                            className={`${activeSidebarTab === "members" ? "text-brand font-bold " : "hover:text-brand/50 hover:cursor-pointer"}`}
                        >
                            Members
                        </button>
                        <button
                            onClick={() => setActiveSidebarTab("activity")}
                            className={`${activeSidebarTab === "activity" ? "text-brand font-bold " : "hover:text-brand/50 hover:cursor-pointer"}`}
                        >
                            Activity
                        </button>
                    </div>

                    {/* Content */}
                    <div className="h-full  overflow-y-auto    rounded-xl shadow-2xl ">
                        {activeSidebarTab === "members" ? (
                            <>
                                <SynapseMembersPanel members={members} />
                            </>
                        ) : (
                            <SynapseActivityPanel
                                isLocalSynapse={isLocalSynapse}
                                synapseMetadata={synapseMetadata}
                                publicKey={user.publicKey}
                            />
                        )}
                    </div>
                </div>
            </div>
        </div>
    );


};

export default SynapseLayout;