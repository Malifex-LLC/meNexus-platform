import './ProfileCard.css';
import '../../hooks/api/useGetProfile.js';
import useGetProfile from "../../hooks/api/useGetProfile.js";
import {useEffect, useState} from "react";
import {NavLink, useNavigate} from "react-router-dom";
import useFollowActions from "../../hooks/api/useFollowActions.js"
import useGetSessionUser from "../../hooks/api/useGetSessionUser.js"
import useCreateNotification from "../../hooks/api/useCreateNotification.js"

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
        <div className="profile-card">
            <div className="profile-card__identity">
                <div className="profile-card__profile-picture">
                    <img src={`http://localhost:3001${profile.profile_picture}`} alt="Profile Picture"/>
                </div>
                <NavLink
                    className="profile-card__display-name"
                    to={`/profile/${profile.handle}`}
                >
                    {profile.display_name}
                </NavLink>
                <NavLink
                    className="profile-card__handle"
                    to={`/profile${profile.handle}`}
                >
                    @{profile.handle}
                </NavLink>
            </div>
            <div className="profile-card__follow-actions">
                <button
                    className="profile-card__follow-button"
                    onClick={isFollowing ? handleUnfollow : handleFollow}
                >
                    {isFollowing ? "Unfollow" : "Follow"}
                </button>
            </div>
        </div>
    )
};

export default ProfileCard;