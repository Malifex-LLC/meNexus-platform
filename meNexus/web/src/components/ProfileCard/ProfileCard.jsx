import '../../api/hooks/useGetProfile.js';
import useGetProfile from "../../api/hooks/useGetProfile.js";
import {useEffect, useState} from "react";
import {Link, NavLink, useNavigate} from "react-router-dom";
import useFollowActions from "../../api/hooks/useFollowActions.js"
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js"
import useCreateNotification from "../../api/hooks/useCreateNotification.js"

const ProfileCard = ({handle}) => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getProfile, data, loading, error} = useGetProfile();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { createNotification } = useCreateNotification();

    const [profile, setProfile] = useState({});
    const [isFollowing, setIsFollowing] = useState(false);
    const [session_user_id, setSession_user_id] = useState(null);
    const navigate = useNavigate();

    useEffect( () => {
        const fetchProfile = async () => {
            try {
                const result = await getProfile(handle);
                setProfile(result);
            } catch (err) {
                console.log(err);
            }
        };

        fetchProfile();

    }, []);

    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                const response = await getSessionUser();

                if (response.status === 200 && response.data.user_id) {
                    setSession_user_id(response.data.user_id);
                } else {
                    console.error("Invalid session, redirecting to login.");
                    navigate ("/login");
                }
            } catch (error) {
                console.error(error);
            }
        }
        fetchSessionUser();
    }, [])

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", profile.user_id);

        const notification = {
            user_id: profile.user_id,
            actor_id: session_user_id,
            resource_type: "FOLLOW",
            resource_id: session_user_id,
            action: "FOLLOW",
        }
        try {
            await followUser(profile.user_id);
            setIsFollowing(true);
            await createNotification(notification);
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", profile.user_id);
        try {
            await unfollowUser(profile.user_id);
            setIsFollowing(false);
        } catch (err) {
            console.error('Error unfollowing user:', err);
        }
    };

    useEffect(() => {
        const fetchFollowStatus = async () => {
            try {
                const isCurrentlyFollowing = await followCheck(profile.user_id);
                setIsFollowing(isCurrentlyFollowing);
            } catch (error) {
                console.error("Error fetching follow status:", error);
            }
        };

        fetchFollowStatus();
    }, [profile.user_id]);

    return (
        <div className="profile-card flex justify-start p-4  my-4 rounded-2xl bg-surface gap-4">
            <div className="profile-card__identity flex gap-8 ">
                <div className="profile-card__profile-picture w-32">
                    <Link
                        className="profile-card__display-name cursor-pointer"
                        to={`/profile/${profile.handle}`}
                    >
                        <img src={profile.profile_picture} alt="Profile Picture"/>
                    </Link>
                </div>
                <div className={`flex flex-col`}>
                    <Link
                        className="profile-card__display-name text-2xl cursor-pointer hover:underline"
                        to={`/profile/${profile.handle}`}
                    >
                        {profile.display_name}
                    </Link>
                    <Link
                        className="profile-card__handle text-xl text-brand cursor-pointer"
                        to={`/profile/${profile.handle}`}
                    >
                        @{profile.handle}
                    </Link>
                    <div className="profile-card__follow-actions py-4">
                        <button
                            className="profile-card__follow-button p-1 px-2 rounded-md bg-brand"
                            onClick={isFollowing ? handleUnfollow : handleFollow}
                        >
                            {isFollowing ? "Unfollow" : "Follow"}
                        </button>
                    </div>
                </div>

            </div>

        </div>
    )
};

export default ProfileCard;