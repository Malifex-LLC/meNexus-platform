// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Post from "../Posting/Post/Post.jsx";
import useFetchRemotePosts from "../../api/hooks/useFetchRemotePosts.js";
import useGetAllPosts from "../../api/hooks/useGetAllPosts.js";
import {useEffect, useState} from "react";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import {useNavigate} from "react-router-dom";
import useGetSynapseMetadata from "../../api/hooks/useGetSynapseMetadata.js";
import useEditPost from "../../api/hooks/useEditPost.js";
import {refreshPosts} from "../../utils/apiUtils.js";
import useDeletePost from "../../api/hooks/useDeletePost.js";
import { FaSortAmountUp } from "react-icons/fa";
import { FiFilter } from "react-icons/fi";
import SortTray from "./SortTray.jsx";
import FilterTray from "./FilterTray.jsx";

const FeedPanel = () => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUser } = useGetUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const { getAllPosts } = useGetAllPosts();
    const [sessionUser, setSessionUser ] = useState({})
    const [user, setUser] = useState({})
    const [posts, setPosts] = useState([]); // State for synapse posts
    const [ showFilterTray, setShowFilterTray ] = useState(false);
    const [ showSortTray, setShowSortTray ] = useState(false);
    const [feedFilters, setFeedFilters] = useState({
        source: 'joined', // 'joined' | 'followed'
        keyword: '',
        sortBy: 'recent', // 'recent' | 'chronological' | 'trending'
    });

    const navigate = useNavigate(); // React Router navigate

    const {
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        handleEdit,
        handleSave,
    } = useEditPost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));

    const { handleDelete } = useDeletePost(() =>
        () => (isLocalSynapse ? refreshPosts(getAllPosts(), setPosts()) :
            refreshPosts(fetchRemotePosts(synapsePublicKey), setPosts())));


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
        // TODO aggregatePosts doesn't handle local vs remote posts to pass to Post object
        const aggregatePosts = async () => {
            console.log('Aggregate Posting called');
            if (!user || !user.synapses) return;

            const localSynapseData = await getSynapseMetadata();
            const allPostPromises = user.synapses.map(async (synapse) => {
                if (synapse === localSynapseData.identity.publicKey) {
                    return await getAllPosts(); // returns array
                } else {
                    return await fetchRemotePosts(synapse); // returns array
                }
            });

            const results = await Promise.all(allPostPromises);
            const combinedPosts = results.flat(); // flatten array of arrays
            setPosts(combinedPosts);
            console.log('Aggregated posts:', combinedPosts);
        };

        aggregatePosts();
    }, [user]);

    const toggleFilterTray = () => {
        setShowFilterTray((prevState) => !prevState);
    }

    const toggleSortTray = () => {
        setShowSortTray((prevState) => !prevState);
    }


    return (
        <div className="flex flex-col p-4 text-3xl bg-background text-foreground text-center ">
            <div className="flex flex-col gap-2 p-2 border-b border-border bg-surface text-lg shadow-lg rounded-xl">
                <div className={'text-3xl'}>
                    Global Feed
                </div>
                <div className={'flex flex-row justify-start gap-8'}>
                    <div className="flex flex-row relative  gap-2">
                        <label className="">Source</label>
                        <button
                            className={'cursor-pointer'}
                            onClick={() => toggleFilterTray()}>
                            <FiFilter />
                        </button>
                    </div>

                    <div className="flex flex-row items-center gap-2">
                        <label className="">Sort By</label>
                        <button
                            className={'cursor-pointer'}
                            onClick={() => toggleSortTray()}
                        >
                            <FaSortAmountUp />
                        </button>
                    </div>

                    <div>
                        {showSortTray && (
                            <div className="absolute  z-50 bg-header-bg/70 rounded-2xl shadow-2xl w-2xl">
                                <SortTray />
                            </div>
                        )}
                    </div>
                    <div>
                        {showFilterTray && (
                            <div className="absolute  z-50 bg-header-bg/70 rounded-2xl shadow-2xl w-2xl">
                                <FilterTray />
                            </div>
                        )}
                    </div>
                </div>

                <div className="flex flex-col">
                    <label className=" mb-1 text-left ">Keyword</label>
                    <input
                        type="text"
                        placeholder="Search posts..."
                        value={feedFilters.keyword}
                        onChange={(e) => setFeedFilters({ ...feedFilters, keyword: e.target.value })}
                        className="bg-background border border-border px-2 py-1 rounded text-foreground"
                    />
                </div>
            </div>

            <div className="flex flex-col text-left space-y-8 mt-8">
                {/* TODO Post object is not passed all required fields. */}
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) => (
                            <div
                                key={index}
                                className={'shadow-2xl'}
                            >
                                <Post
                                    key={index}
                                    postId={post.post_id}
                                    publicKey={post.public_key}
                                    session_user_id={user.publicKey}
                                    date={post.created_at}
                                    content={post.content}
                                    comments={0}
                                    likes={0}
                                    onDelete={() => handleDelete(post.post_id)}
                                    onEdit={() => handleEdit(post.post_id, posts)}
                                    isEditing={editingPostId === post.post_id}
                                    editedContent={editedPostContent}
                                    onContentChange={(event) =>
                                        setEditedPostContent(event.target.value)
                                    }
                                    onSave={handleSave}
                                />
                            </div>
                        ))
                ) : (
                    <div>No posts to show.</div>
                )}
            </div>

        </div>
    );
}

export default FeedPanel;