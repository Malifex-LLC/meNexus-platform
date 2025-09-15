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

const FeedPanel = ({user, localSynapseMetadata}) => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUser } = useGetUser();
    const { getSynapseMetadata } = useGetSynapseMetadata();
    const { fetchRemotePosts, loading: synapsePostsLoading, error: synapsePostsError } = useFetchRemotePosts();
    const { getAllPosts } = useGetAllPosts();
    const [sessionUser, setSessionUser ] = useState({})
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
        // TODO aggregatePosts doesn't handle local vs remote posts to pass to Post object
        const aggregatePosts = async () => {
            console.log('Aggregate Posting called');
            if (!user || !user.synapses) return;

            const allPostPromises = user.synapses.map(async (synapse) => {
                if (synapse === localSynapseMetadata.identity.publicKey) {
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
        <div className="flex flex-col flex-1  w-full h-full text-foreground bg-surface/70 xl:rounded-xl border border-border">
            <div className="flex flex-col justify-around p-4 gap-4 bg-surface border-b border-border rounded-t-xl text-2xl text-foreground shadows-2xl ">
                <button className={`text-brand font-bold `}>
                    Global Feed
                </button>
            </div>
            <div className="flex flex-row gap-2 p-4  w-full border-b border-border bg-background text-lg shadow-lg">
                <div className="flex flex-col w-full">
                    <input
                        type="text"
                        placeholder="Search posts..."
                        value={feedFilters.keyword}
                        onChange={(e) => setFeedFilters({ ...feedFilters, keyword: e.target.value })}
                        className="bg-surface border border-border px-2 py-1 rounded text-foreground text-sm md:text-lg"
                    />
                </div>
                <div className={'flex flex-col w-full items-center justify-center text-sm md:text-lg'}>
                    <div className={`flex flex-row w-full`}>
                        <div className="flex flex-row w-full items-center justify-center gap-2">
                            <label className="">Source</label>
                            <button
                                className={'flex w-full cursor-pointer hover:text-brand/60'}
                                onClick={() => toggleFilterTray()}>
                                <FiFilter />
                            </button>
                            <div className={`absolute z-50 mt-33`}>
                                {showFilterTray && (
                                    <div className="px-4 bg-header-bg/70 rounded-xl shadow-2xl backdrop-blur-xs border border-border rounded-xl">
                                        <FilterTray />
                                    </div>
                                )}
                            </div>
                        </div>
                        <div className="flex flex-row w-full items-center justify-center gap-2">
                            <label className="">Sort By</label>
                            <button
                                className={'cursor-pointer hover:text-brand/60'}
                                onClick={() => toggleSortTray()}
                            >
                                <FaSortAmountUp />
                            </button>
                            <div className={`absolute z-50 mt-40`}>
                                {showSortTray && (
                                    <div className="px-4 bg-surface/70 rounded-xl shadow-2xl backdrop-blur-xs border border-border rounded-xl">
                                        <SortTray />
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                </div>

            </div>
            <div className="flex flex-col overflow-y-auto text-left space-y-8 mt-4 p-4 ">
                {/* TODO Post object is not passed all required fields. */}
                {posts.length > 0 ? (
                    posts
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .map((post, index) =>  (

                            <div
                                key={index}
                                className={'shadow-2xl'}
                            >
                                <Post
                                    key={index}
                                    isLocalSynapse={post.synapsePublicKey === localSynapseMetadata.identity.publicKey}
                                    postId={post.post_id}
                                    publicKey={post.public_key}
                                    sessionPublicKey={user.publicKey}
                                    synapseUrl={post.synapseUrl}
                                    date={post.created_at}
                                    content={post.content}
                                    mediaUrl={post.media_url}
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
                    <div className={`flex flex-col p-4 text-2xl text-center`}>No posts to show.</div>
                )}
            </div>
        </div>
    );
}

export default FeedPanel;