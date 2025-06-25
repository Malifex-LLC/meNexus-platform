import useGetProfile from "../../api/hooks/useGetProfile.js";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import useGetFollowerCount from "../../api/hooks/useGetFollowerCount.js";
import useGetFollowingCount from "../../api/hooks/useGetFollowingCount.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import FollowedUsersPanel from "../FollowedUsersPanel/FollowedUsersPanel.jsx";
import JoinedSynapsesPanel from "../JoinedSynapsesPanel/JoinedSynapsesPanel.jsx";
import {IoLocationSharp} from "react-icons/io5";

const ControlPanel = ({user}) => {
    const { getFollowerCount, loading: followerCountLoading, error: followerCountError } = useGetFollowerCount();
    const { getFollowingCount, loading: followingCountLoading, error: followingCountError } = useGetFollowingCount();

    const [followerCount, setFollowerCount] = useState(0);
    const [followingCount, setFollowingCount] = useState(0);

    useEffect(() => {
        const fetchFollowerCount = async () => {
            try {
                const fetchedFollowerCount = await getFollowerCount(user.publicKey);
                setFollowerCount(fetchedFollowerCount.result);
            } catch (error) {
                console.error("Error fetching follower count:", error);
            }
        }
        fetchFollowerCount();
    }, []);

    useEffect(() => {
        const fetchFollowingCount = async () => {
            try {
                const fetchedFollowingCount = await getFollowingCount(user.publicKey);
                setFollowingCount(fetchedFollowingCount.result);
            } catch (error) {
                console.error("Error fetching following count:", error);
            }
        }
        fetchFollowingCount();
    }, []);


    return (
        <div className={`relative flex flex-col p-4 items-center shadow-2xl`}>
            <div className={`p-24 md:p-12 xl:p-24 w-full relative top-2 bg-gradient-to-b from-background via-primary to-surface backdrop-blur-md rounded-2xl`}/>
            <div className=" relative z-1  flex flex-col px-4 pb-0 w-full -top-8  justify-center rounded-xl shadow-md
            bg-surface text-foreground">
                <div className={`flex  justify-center`}>
                    <div className={`flex flex-col relative  items-center text-md`}>
                        <p className={`px-4 lg:px-2  xl:px-4`}>{followerCount}</p>
                        <p className={`px-4 lg:px-2  xl:px-4`}>Followers</p>
                    </div>
                    <img
                        className={`relative -top-16 w-32 mb-0 pb-0`}
                        src={`${import.meta.env.VITE_API_BASE_URL}${user.profilePicture}`}
                        alt={`${user.displayName}'s profile picture`}
                    />
                    <div className={`flex flex-col items-center text-md`}>
                        <p className={`px-4 lg:px-2 xl:px-4 `}>{followingCount}</p>
                        <p className={`px-4 lg:px-2 xl:px-4 `}>Following</p>
                    </div>
                </div>
                <div className={`relative flex flex-col py-2 md:-top-8 -top-16 items-center text-foreground`}>
                    <p className={`md:text-2xl xl:text-3xl`}>{user.displayName}</p>
                    <p className={`md:text-lg xl:text-xl text-brand`}>@{user.handle}</p>
                </div>
            </div>
            <div className={'flex flex-col w-full'}>
                <div className={'border border-border rounded-xl mb-4'}>
                    <JoinedSynapsesPanel synapses={user.synapses} />
                </div>
                <div className={'border border-border rounded-xl'}>
                    <FollowedUsersPanel following={user.following} />
                </div>
            </div>
        </div>
    );
};

export default ControlPanel;