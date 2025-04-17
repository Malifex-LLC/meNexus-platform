import useGetProfile from "../../api/hooks/useGetProfile.js";
import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useFollowActions from "../../api/hooks/useFollowActions.js";

const SocialPanel = () => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();

    const [profile, setProfile] = useState({});
    const { handle } = useParams();
    const [currentHandle, setCurrentHandle] = useState(handle || null);
    const [session_user_id, setSession_user_id] = useState(null);
    const [session_user_handle, setSession_user_handle] = useState(null);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const [isFollowing, setIsFollowing] = useState(false);

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
                        setSession_user_id(response.data.user_id);
                        setSession_user_handle(response.data.handle);
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
                setSession_user_id(response.data.user_id);
                setCurrentHandle(handle);
                setSession_user_handle(response.data.handle);
                setIsHandleSet(true);
            }
        };
        fetchSessionUser();
    }, [handle, isHandleSet]);

    // Fetch profile and posts once the current handle is determined
    useEffect(() => {
        if (currentHandle && isHandleSet) {
            const fetchData = async () => {
                try {
                    console.log("Fetching profile and posts for handle:", currentHandle);
                    const [profileData] = await Promise.all([
                        getProfile(currentHandle),
                    ]);
                    console.log("Profile Data after getProfile() fetching is:", profileData);
                    setProfile(profileData);

                    const isCurrentlyFollowing = await followCheck(profileData.user_id);
                    console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                    setIsFollowing(isCurrentlyFollowing);
                } catch (error) {
                    console.error("Error fetching data:", error);
                }
            };

            fetchData();
        }
    }, [currentHandle, isHandleSet]);


    return (
        <div className={`relative flex flex-col h-screen w-sm p-4 items-center`}>
            <div className={`p-24 w-full relative top-2 bg-gradient-to-b from-background via-primary to-surface backdrop-blur-md rounded-2xl`}/>
            <div className=" relative z-1  flex flex-col px-4 pb-0 w-full -top-8  justify-center rounded-2xl
            bg-surface text-foreground">
                <div className={`flex  justify-center`}>
                    <div className={`flex flex-col relative  items-center`}>
                        <p className={`px-4`}>420</p>
                        <p className={`px-4`}>Followers</p>
                    </div>
                    <img
                        className={`relative -top-16  w-32 mb-0 pb-0`}
                        src={profile.profile_picture}
                        alt={`${profile.display_name}'s profile picture`}
                    />
                    <div className={`flex flex-col items-center`}>
                        <p className={`px-4`}>69</p>
                        <p className={`px-4`}>Following</p>
                    </div>
                </div>
                <div className={`relative flex flex-col py-2 -top-16 items-center text-foreground`}>
                    <p className={`text-3xl`}>{profile.display_name}</p>
                    <p className={`text-xl text-brand`}>@{currentHandle}</p>
                </div>
                <div className={`relative -top-8 flex flex-col  items-center`}>
                    <p>{profile.profile_bio}</p>
                </div>

            </div>
            <div className={`flex text-foreground gap-4`}>

            </div>


        </div>
    );
};

export default SocialPanel;