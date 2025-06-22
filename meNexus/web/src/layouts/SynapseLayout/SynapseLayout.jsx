import Header from "../../components/Header/Header.jsx";
import {useEffect, useState} from "react";
import useGetUser from "../../api/hooks/useGetUser.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import {useNavigate, useParams} from "react-router-dom";
import SocialPanel from "../../components/SocialPanel/SocialPanel.jsx";
import {useSwipeable} from "react-swipeable";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import SynapseUsersPanel from "../../components/SynapseUsersPanel/SynapseUsersPanel.jsx";
import useFetchRemoteSynapseMetadata from "../../api/hooks/useFetchRemoteSynapseMetadata.js";
import ChatPanel from "../../components/Chatting/ChatPanel/ChatPanel.jsx";
import PostsPanel from "../../components/Posts/PostsPanel/PostsPanel.jsx";
import useFetchRemotePosts from "../../api/hooks/useFetchRemotePosts.js";
import SynapseControlBar from "../../components/SynapseControlBar/SynapseControlBar.jsx";
import JoinedSynapsesPanel from "../../components/JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useGetAllPosts from "../../api/hooks/useGetAllPosts.js";

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
    const [posts, setPosts] = useState([]); // State for synapse posts
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const [activeSidebarTab, setActiveSidebarTab] = useState("activity"); // or "chat"
    const [activeMiddlebarTab, setActiveMiddlebarTab] = useState("feed");

    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { getAllPosts } = useGetAllPosts();

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
                } else {
                    try {
                        const synapseMetadataResponse = await fetchRemoteSynapseMetadata(synapsePublicKey);
                        setSynapseMetadata(synapseMetadataResponse);
                        setIsLocalSynapse(false);
                    } catch (error) {
                        console.error("Error fetching remote Synapse data: ", error);
                    }
                }
            } catch (error) {
                console.error("Error fetching local Synapse data:", error);
            }
        };
        fetchSynapseMetadata();
    },[]);

    useEffect(() => {
        const fetchSynapsePosts = async () => {
            if (!synapsePublicKey) {
                return;
            }
            if (isLocalSynapse) {
                try {
                    const synapsePostsData = await getAllPosts();
                    setPosts(synapsePostsData);

                } catch (error) {
                    console.error("Error fetching Synapse posts: ", error);
                }
            } else {
                try {
                    const synapsePostsData = await fetchRemotePosts(synapsePublicKey);
                    setPosts(synapsePostsData);
                } catch (error) {
                    console.error("Error fetching remote Synapse posts: ", error);
                }
            }

        };
        fetchSynapsePosts();
    },[isLocalSynapse, synapseMetadata]);


    if (!user || !user.publicKey) {
        return <>Loading dashboard...</>;
    }

    if (synapsePostsLoading) {
        return <div>Loading Synapse Posts...</div>
    }

    if (synapsePostsError) {
        return <div>Error Loading Synapse Posts: {synapsePostsError.message}</div>
    }

    return (
        <div className="home-layout h-screen flex flex-col bg-background">
            {/* Header */}
            <div className="sticky top-0 z-50 border-b border-border">
                <Header />
            </div>

            {/* Main Grid */}
            <div className="flex-1 w-full lg:grid lg:grid-cols-12 overflow-hidden">

                {/* Social Panel (Left Column) */}
                <div className="hidden lg:flex flex-col pt-17 border-r border-border lg:col-span-2 overflow-y-auto">
                    <div className="">
                        <SocialPanel user={user} />
                    </div>

                </div>




                {/* Main Content (Center Column) */}
                <div className="flex flex-col h-full w-full lg:col-span-8 overflow-hidden">

                    {/* Synapse Control Bar */}
                    <div className="border-b border-border">
                        <SynapseControlBar synapseMetadata={synapseMetadata} />
                    </div>

                    {/* Scrollable PostsPanel */}
                    <div className={'hidden lg:flex flex-col  border-l border-border h-full lg:col-span-8 '}>
                        {/* Tab Switcher */}
                        <div className={'flex justify-around border-b bg-background border-border p-2 text-sm text-foreground'}>
                            <button
                                className={`w-full h-full border-r border-border hover:cursor-pointer
                                ${activeMiddlebarTab === "feed" ? "text-brand font-bold" : "hover:text-brand"}`}
                                onClick={() => setActiveMiddlebarTab("feed")}
                            >
                                Feed
                            </button>
                            <button
                                className={` w-full h-full hover:cursor-pointer
                                ${activeMiddlebarTab === "chat" ? "text-brand font-bold" : "hover:text-brand"}`}
                                onClick={() => setActiveMiddlebarTab("chat")}
                            >
                                Chat
                            </button>
                        </div>
                        {/* Content */}

                        <div className={'flex h-full'}>
                            {activeMiddlebarTab === "feed" ? (
                                <div className={'flex-1 overflow-y-auto'}>
                                    <PostsPanel
                                        isLocalSynapse={isLocalSynapse}
                                        publicKey={sessionUser.publicKey}
                                        synapsePublicKey={synapsePublicKey}
                                        posts={posts}
                                        setPosts={setPosts}
                                    />
                                </div>
                            ) : activeMiddlebarTab === "chat" ? (
                                <div className={'flex-1 overflow-hidden'}>
                                    <ChatPanel synapsePublicKey={synapsePublicKey} />
                                </div>

                            ) : (
                                <div className="flex flex-col h-full w-full lg:col-span-6 ">
                                    unknown tab content
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* Right Column (Activity Feed + Users) */}
                <div className="hidden lg:flex flex-col pt-17 border-l border-border h-full lg:col-span-2 overflow-hidden">
                    {/* Tab Switcher */}
                    <div className="flex justify-around border-b border-border p-2 text-sm text-foreground">
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
                    <div className="flex-1 overflow-y-auto p-4">
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