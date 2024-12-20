import './ProfileCard.css';
import '../../api/hooks/useGetProfile.js';
import useGetProfile from "../../api/hooks/useGetProfile.js";
import {useEffect, useState} from "react";
import {NavLink} from "react-router-dom";
import useFollowActions from "../../api/hooks/useFollowActions.js"

const ProfileCard = ({handle}) => {
    const [profile, setProfile] = useState({});
    const [isFollowing, setIsFollowing] = useState(false);

    const {getProfile, data, loading, error} = useGetProfile();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();

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

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", profile.user_id);
        try {
            await followUser(profile.user_id);
            setIsFollowing(true);
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