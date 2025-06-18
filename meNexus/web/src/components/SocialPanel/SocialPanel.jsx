import useGetProfile from "../../api/hooks/useGetProfile.js";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import useGetFollowerCount from "../../api/hooks/useGetFollowerCount.js";
import useGetFollowingCount from "../../api/hooks/useGetFollowingCount.js";
import useGetUser from "../../api/hooks/useGetUser.js";

const SocialPanel = () => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUser, loading: userLoading, error: userError } = useGetUser();
    const { getFollowerCount, loading: followerCountLoading, error: followerCountError } = useGetFollowerCount();
    const { getFollowingCount, loading: followingCountLoading, error: followingCountError } = useGetFollowingCount();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();

    const { handle } = useParams();
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [displayName, setDisplayName] = useState('');
    const [bio, setBio] = useState('');
    const [profilePicture, setProfilePicture] = useState('');
    const [sessionUserPublicKey, setsessionUserPublicKey] = useState(null);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const [isFollowing, setIsFollowing] = useState(false);
    const [followerCount, setFollowerCount] = useState(0);
    const [followingCount, setFollowingCount] = useState(0);

    const navigate = useNavigate();

    // Redirect from /profile to /profile/:handle if no handle is provided
    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!handle && !isHandleSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();

                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setCurrentHandle(response.data.handle);
                        setIsHandleSet(true);
                        setsessionUserPublicKey(response.data.publicKey);

                    } else {
                        console.error("Invalid session, redirecting to login.");
                        navigate('/login');
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                    navigate('/login');
                }
            } else if (handle) {
                const response = await getSessionUser();
                setsessionUserPublicKey(response.data.publicKey);
                setCurrentHandle(handle);
                setIsHandleSet(true);
            }
        };
        fetchSessionUser();
    }, [handle, isHandleSet]);

    useEffect( () => {
        const fetchUserData = async () => {
            try {
                console.log("Fetching userData for publicKey: ", sessionUserPublicKey)
                const userData = await getUser(sessionUserPublicKey)
                console.log('userData: ', userData)
                setDisplayName(userData.displayName)
                setBio(userData.bio);
                setProfilePicture(userData.profilePicture);
                const isCurrentlyFollowing = await followCheck(userData.publicKey);
                console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                setIsFollowing(isCurrentlyFollowing);
            } catch (error) {
                console.error("Error fetching userData:", error);
            }
        }
        fetchUserData();
    }, [sessionUserPublicKey])

    useEffect(() => {
        const fetchFollowerCount = async () => {
            try {
                const fetchedFollowerCount = await getFollowerCount(sessionUserPublicKey);
                setFollowerCount(fetchedFollowerCount.result);
            } catch (error) {
                console.error("Error fetching follower count:", error);
            }
        }
        fetchFollowerCount();
    }, [sessionUserPublicKey]);

    useEffect(() => {
        const fetchFollowingCount = async () => {
            try {
                const fetchedFollowingCount = await getFollowingCount(sessionUserPublicKey);
                setFollowingCount(fetchedFollowingCount.result);
            } catch (error) {
                console.error("Error fetching following count:", error);
            }
        }
        fetchFollowingCount();
    }, [sessionUserPublicKey]);


    return (
        <div className={`relative flex flex-col h-screen  p-4 items-center`}>
            <div className={`p-24 md:p-12 xl:p-24 w-full relative top-2 bg-gradient-to-b from-background via-primary to-surface backdrop-blur-md rounded-2xl`}/>
            <div className=" relative z-1  flex flex-col px-4 pb-0 w-full -top-8  justify-center rounded-2xl
            bg-surface text-foreground">
                <div className={`flex  justify-center`}>
                    <div className={`flex flex-col relative  items-center text-xl lg:text-xs xl:text-md 2xl:text-2xl`}>
                        <p className={`px-4 lg:px-2  xl:px-4`}>{followerCount}</p>
                        <p className={`px-4 lg:px-2  xl:px-4`}>Followers</p>
                    </div>
                    <img
                        className={`relative -top-16 w-48 lg:w-16 lg:-top-8 xl:w-32 mb-0 pb-0`}
                        src={profilePicture}
                        alt={`${displayName}'s profile picture`}
                    />
                    <div className={`flex flex-col items-center text-xl lg:text-xs xl:text-md 2xl:text-2xl`}>
                        <p className={`px-4 lg:px-2 xl:px-4 `}>{followingCount}</p>
                        <p className={`px-4 lg:px-2 xl:px-4 `}>Following</p>
                    </div>
                </div>
                <div className={`relative flex flex-col py-2 md:-top-8 -top-16 items-center text-foreground`}>
                    <p className={`md:text-2xl xl:text-3xl`}>{displayName}</p>
                    <p className={`md:text-lg xl:text-xl text-brand`}>@{currentHandle}</p>
                </div>
                <div className={`relative text-lg md:text-sm -top-8 flex flex-col  items-center`}>
                    <p>{bio}</p>
                </div>

            </div>
            <div className={`flex text-foreground gap-4`}>

            </div>


        </div>
    );
};

export default SocialPanel;