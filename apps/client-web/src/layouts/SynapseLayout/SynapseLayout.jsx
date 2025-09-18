// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import React, {useEffect, useState} from "react";
import { useOutletContext, useParams } from "react-router-dom";
import {useSwipeable} from "react-swipeable";
import ChatPanel from "../../components/Chatting/ChatPanel/ChatPanel.jsx";
import PostsPanel from "../../components/Posting/PostingPanel/PostsPanel.jsx";
import SynapseInfoTray from "../../components/SynapseInfoTray/SynapseInfoTray.jsx";
import SynapseControlBarTray from "../../components/SynapseControlBar/SynapseControlBarTray.jsx";
import useGetBoardPosts from "../../api/hooks/useGetBoardPosts.js";
import useFetchRemoteBoardPosts from "../../api/hooks/useFetchRemoteBoardPosts.js";
import useGetSynapsePostBoards from "../../api/hooks/useGetSynapsePostBoards.js";
import useFetchRemoteSynapsePostBoards from "../../api/hooks/useFetchRemoteSynapsePostBoards.js";
import useGetChannelChatMessages from "../../api/hooks/useGetChannelChatMessages.js";
import useGetSynapseChatChannels from "../../api/hooks/useGetSynapseChatChannels.js";
import useFetchRemoteSynapseChatChannels from "../../api/hooks/useFetchRemoteSynapseChatChannels.js";
import useFetchRemoteChannelChats from "../../api/hooks/useFetchRemoteChannelChats.js";
import useGetSynapseMembers from "../../api/hooks/useGetSynapseMembers.js";
import useFetchRemoteSynapseMembers from "../../api/hooks/useFetchRemoteSynapseMembers.js";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import { LuPanelLeftClose } from "react-icons/lu";
import { LuPanelRightClose } from "react-icons/lu";

function useRootContext() {
    return useOutletContext(); // { sessionUser, user, localSynapseMetadata }
}

const SynapseLayout =({ children }) => {
    const { sessionUser, user, localSynapseMetadata } = useRootContext();
    const { synapsePublicKey } = useParams(); // Extract synapsePublicKey from the URL (if available)
    const [isLocalSynapse, setIsLocalSynapse] = useState(true);
    const [synapseMetadata, setSynapseMetadata] = useState(null);
    const [members, setMembers] = useState(null);
    const [boards, setBoards] = useState(null);
    const [activeBoard, setActiveBoard] = useState(null);
    const [posts, setPosts] = useState([]); // State for synapse posts
    const [channels, setChannels] = useState(null);
    const [activeChannel, setActiveChannel] = useState(null);
    const [chatMessages, setChatMessages] = useState([]);
    const [activeSidebarTab, setActiveSidebarTab] = useState("activity");
    const [activeMiddlebarTab, setActiveMiddlebarTab] = useState("feed");
    const [isSynapseInfoTrayOpen, setIsSynapseInfoTrayOpen] = useState(false);
    const [isSynapseControlBarTrayOpen, setIsSynapseControlBarTrayOpen] = useState(false);

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

    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => {
            if (!isSynapseControlBarTrayOpen && !isSynapseInfoTrayOpen) setIsSynapseInfoTrayOpen(true);
        },
        onSwipedRight: () => {
            if (!isSynapseControlBarTrayOpen && !isSynapseInfoTrayOpen) setIsSynapseControlBarTrayOpen(true);
        },
        trackTouch: true,
        trackMouse: true,
        preventDefaultTouchmoveEvent: true,
        delta: 50, // min px before triggering
    });

    useEffect(() => {
        const fetchSynapseMetadata = async () => {
            try {
                if (localSynapseMetadata.identity.publicKey === synapsePublicKey) {
                    setSynapseMetadata(localSynapseMetadata);
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
    },[synapsePublicKey, isLocalSynapse, activeBoard]);

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
    }, [synapsePublicKey, isLocalSynapse, activeChannel])

    // Close drawers on ESC
    useEffect(() => {
        if (!(isSynapseControlBarTrayOpen || isSynapseInfoTrayOpen)) return;
        const onKey = (e) => e.key === 'Escape' && (setIsSynapseControlBarTrayOpen(false), setIsSynapseInfoTrayOpen(false));
        window.addEventListener('keydown', onKey);
        return () => window.removeEventListener('keydown', onKey);
    }, [isSynapseControlBarTrayOpen, isSynapseInfoTrayOpen]);

    // Lock body scroll while any drawer is open
    useEffect(() => {
        document.body.style.overflow = (isSynapseControlBarTrayOpen || isSynapseInfoTrayOpen) ? 'hidden' : '';
    }, [isSynapseControlBarTrayOpen, isSynapseInfoTrayOpen]);

    // Set isXL based off window size for mounting components only once conditionally
    function useIsXL() {
        const [isXL, setIsXL] = React.useState(
            typeof window !== 'undefined' && window.matchMedia('(min-width: 1280px)').matches
        );
        React.useEffect(() => {
            if (typeof window === 'undefined') return;
            const mql = window.matchMedia('(min-width: 1280px)');
            const onChange = (e) => setIsXL(e.matches);
            mql.addEventListener?.('change', onChange);
            return () => mql.removeEventListener?.('change', onChange);
        }, []);
        return isXL;
    }

    const isXL = useIsXL();

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
        <div className="flex flex-col h-[100dvh]">
            {/* Header */}
            <div className="sticky top-0 z-50 border-b border-border bg-background pt-16">
                <div className="flex items-center justify-between">
                    {/* Left tray button (Browse) */}
                    <button
                        className="xl:hidden py-2 text-3xl text-foreground hover:text-brand/60 hover:cursor-pointer"
                        aria-label="Open left menu"
                        aria-expanded={isSynapseControlBarTrayOpen}
                        aria-controls="synapse-drawer"
                        onClick={() => setIsSynapseControlBarTrayOpen(true)}
                    >
                        <LuPanelRightClose />
                    </button>

                    {/* Center label */}
                    {!isXL && (
                        <div className="flex w-full justify-center text-2xl text-brand font-montserrat">
                            {activeMiddlebarTab === 'feed' ? <div>{activeBoard}</div> : <div>{activeChannel}</div>}
                        </div>
                    )}

                    {/* Right tray button (Info) */}
                    <div className="flex items-center gap-2">
                        <button
                            className="xl:hidden py-2 text-3xl text-foreground hover:text-brand/60 hover:cursor-pointer"
                            aria-label="Open right menu"
                            aria-expanded={isSynapseInfoTrayOpen}
                            aria-controls="people-drawer"
                            onClick={() => setIsSynapseInfoTrayOpen(true)}
                        >
                            <LuPanelLeftClose />
                        </button>
                    </div>
                </div>
            </div>

            {/* ===== Mobile Backdrop (shared) ===== */}
            <div
                className={`fixed inset-0 z-[55] xl:hidden transition
              ${ (isSynapseControlBarTrayOpen || isSynapseInfoTrayOpen)
                    ? 'pointer-events-auto bg-surface/40 backdrop-blur-sm'
                    : 'pointer-events-none bg-transparent' }`}
                {...swipeHandlers}
                aria-label="Close menus"
                onClick={() => { setIsSynapseControlBarTrayOpen(false); setIsSynapseInfoTrayOpen(false); }}
            />

            {/* ===== Left Tray (Browse) ===== */}
            {!isXL && (
                <div
                    id="synapse-control-bar-tray"
                    role="dialog"
                    aria-modal="true"
                    className={`fixed inset-y-0 left-0 w-80 max-w-full transform transition-transform duration-300 ease-in-out bg-surface/95 border-r border-border shadow-2xl z-[55] xl:hidden 
                    ${isSynapseControlBarTrayOpen ? 'translate-x-0' : '-translate-x-full'}`}
                >
                    <div className="flex items-center justify-between p-3 border-b border-border text-foreground">
                        <h2 className="text-lg font-montserrat font-semibold">Browse</h2>
                        <button
                            className="p-2 text-2xl hover:text-brand/60 hover:cursor-pointer"
                            aria-label="Close left menu"
                            onClick={() => setIsSynapseControlBarTrayOpen(false)}
                        >
                            <LuPanelLeftClose />
                        </button>
                    </div>
                    <div className="h-[calc(100%-3.25rem)] overflow-hidden">
                        <SynapseControlBarTray
                            user={user}
                            activeMiddlebarTab={activeMiddlebarTab}
                            setActiveMiddlebarTab={setActiveMiddlebarTab}
                            boards={boards}
                            activeBoard={activeBoard}
                            setActiveBoard={setActiveBoard}
                            channels={channels}
                            activeChannel={activeChannel}
                            setActiveChannel={setActiveChannel}
                        />
                    </div>
                </div>
            )}

            {/* ===== Right Tray (Info) ===== */}
            {!isXL && (
                <div
                    id="synapse-info-tray"
                    role="dialog"
                    aria-modal="true"
                    className={`fixed inset-y-0 right-0 w-80 max-w-[90vw] transform transition-transform duration-300 ease-in-out bg-surface/95 border-l border-border shadow-2xl z-[60] xl:hidden 
                    ${isSynapseInfoTrayOpen ? 'translate-x-0' : 'translate-x-full'}`}
                >
                    <div className="flex items-center justify-between p-3 border-b border-border text-foreground">
                        <h2 className="text-lg font-montserrat font-semibold">Info</h2>
                        <button
                            className="p-2 text-2xl hover:text-brand/60 hover:cursor-pointer"
                            aria-label="Close right menu"
                            onClick={() => setIsSynapseInfoTrayOpen(false)}
                        >
                            <LuPanelRightClose />
                        </button>
                    </div>
                    <div className="h-[calc(100%-3.25rem)] overflow-hidden">
                        <SynapseInfoTray
                            activeSidebarTab={activeSidebarTab}
                            setActiveSidebarTab={setActiveSidebarTab}
                            members={members}
                            isLocalSynapse={isLocalSynapse}
                            synapseMetadata={synapseMetadata}
                            user={user}
                        />
                    </div>
                </div>
            )}

            {/* ===== Main Grid / Content ===== */}
            <div className="flex flex-1 xl:p-4 gap-4 min-h-0 w-full overflow-hidden xl:grid xl:grid-cols-12"
                 {...swipeHandlers}
            >
                {/* Left Column (xl+): same as left drawer */}
                {isXL && (
                    <div className="hidden xl:flex xl:col-span-3 rounded-xl border border-border overflow-hidden shadow-2xl">
                        <SynapseControlBarTray
                            user={user}
                            activeMiddlebarTab={activeMiddlebarTab}
                            setActiveMiddlebarTab={setActiveMiddlebarTab}
                            boards={boards}
                            activeBoard={activeBoard}
                            setActiveBoard={setActiveBoard}
                            channels={channels}
                            activeChannel={activeChannel}
                            setActiveChannel={setActiveChannel}
                        />
                    </div>
                )}

                {/* Center Column */}
                <div className="flex flex-col flex-1 min-h-0 w-full xl:col-span-6 overflow-hidden">
                    <div className="flex-1 min-h-0 overflow-y-auto">
                        {activeMiddlebarTab === 'feed' ? (
                            <div className="w-full h-full xl:rounded-xl shadow-2xl">
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
                        ) : (
                            <div className="flex w-full h-full shadow-2xl xl:rounded-xl">
                                <ChatPanel
                                    isLocalSynapse={isLocalSynapse}
                                    synapseMetadata={synapseMetadata}
                                    publicKey={sessionUser.publicKey}
                                    channels={channels}
                                    activeChannel={activeChannel}
                                    setActiveChannel={setActiveChannel}
                                    chatMessages={chatMessages}
                                    setChatMessages={setChatMessages}
                                />
                            </div>
                        )}
                    </div>
                </div>

                {/* Right Column (xl+): same as right drawer */}
                {isXL && (
                    <div className="hidden xl:flex flex-col w-full rounded-xl xl:col-span-3 overflow-hidden bg-surface/70 border border-border">
                        <SynapseInfoTray
                            activeSidebarTab={activeSidebarTab}
                            setActiveSidebarTab={setActiveSidebarTab}
                            members={members}
                            isLocalSynapse={isLocalSynapse}
                            synapseMetadata={synapseMetadata}
                            user={user}
                        />
                    </div>
                )}
            </div>
        </div>
    );
};

export default SynapseLayout;