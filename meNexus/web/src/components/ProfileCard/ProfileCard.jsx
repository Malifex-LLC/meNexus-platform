import '../../api/hooks/useGetProfile.js';
import {useEffect, useState} from "react";
import {Link, NavLink, useNavigate} from "react-router-dom";
import useFollowActions from "../../api/hooks/useFollowActions.js";
import useGetSessionUser from "../../api/hooks/useGetSessionUser.js";
import useGetUser from "../../api/hooks/useGetUser.js";
import useCreateNotification from "../../api/hooks/useCreateNotification.js";

const ProfileCard = ({publicKey, handle}) => {
    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const {getUser } = useGetUser();
    const { followUser, unfollowUser, followCheck, loading: followUserLoading, error: followUserError } = useFollowActions();
    const { createNotification } = useCreateNotification();

    const [isFollowing, setIsFollowing] = useState(false);
    const [sessionPublicKey, setSessionPublicKey] = useState(null);
    const [user, setUser] = useState(null)
    const navigate = useNavigate();

    useEffect( () => {
        const fetchUserData = async () => {
            try {
                const userData = await getUser(publicKey);
                console.log('userData in ProfileCard: ', userData)
                setUser(userData)
            } catch (err) {
                console.log(err);
            }
        };
        fetchUserData(publicKey);

    }, [publicKey]);

    useEffect(() => {
        const fetchSessionUser = async () => {
            try {
                const response = await getSessionUser();
                if (response.status === 200 && response.data.publicKey) {
                    setSessionPublicKey(response.data.publicKey);
                } else {
                    console.error("Invalid session, redirecting to login.");
                    navigate ("/login");
                }
            } catch (error) {
                console.error(error);
            }
        }
        fetchSessionUser();
    }, [navigate])

    const handleFollow = async () => {
        console.log("handleFollow for followed_id: ", user.publicKey);

        const notification = {
            public_key: user.publicKey,
            actor_public_key: sessionPublicKey,
            resource_type: "FOLLOW",
            resource_id: sessionPublicKey,
            action: "FOLLOW",
        }
        try {
            await followUser(user.publicKey);
            setIsFollowing(true);
            await createNotification(notification);
        } catch (err) {
            console.log('Error following user', err);
        }
    };

    const handleUnfollow = async () => {
        console.log("handleUnFollow for followed_id: ", user.publicKey);
        try {
            await unfollowUser(user.publicKey);
            setIsFollowing(false);
        } catch (err) {
            console.error('Error unfollowing user:', err);
        }
    };

    useEffect(() => {
        if(!user?.publicKey || !sessionPublicKey) {
            return;
        }
        const fetchFollowStatus = async () => {
            try {
                const isCurrentlyFollowing = await followCheck(user.publicKey);
                console.log('profileCard calling followCheck for followedPublicKey: ', user.publicKey);
                console.log("isCurrentlyFollowing: ", isCurrentlyFollowing);
                setIsFollowing(isCurrentlyFollowing);
            } catch (error) {
                console.error("Error fetching follow status:", error);
            }
        };
        fetchFollowStatus();
    }, [user?.publicKey, sessionPublicKey]);

    if (!user) {
        return <div>Loading user...</div>
    }

    return (
        <div className="profile-card flex justify-start p-4  my-4 rounded-2xl bg-surface gap-4">
            <div className="profile-card__identity flex gap-8 ">
                <div className="profile-card__profile-picture w-32">
                    <Link
                        className="profile-card__display-name cursor-pointer"
                        to={`/profile/${user.publicKey}`}
                    >
                        <img src={user.profilePicture} alt="Profile Picture"/>
                    </Link>
                </div>
                <div className={`flex flex-col`}>
                    <Link
                        className="profile-card__display-name text-2xl cursor-pointer hover:underline"
                        to={`/profile/${user.handle}`}
                    >
                        {user.displayName}
                    </Link>
                    <Link
                        className="profile-card__handle text-xl text-brand cursor-pointer"
                        to={`/profile/${user.handle}`}
                    >
                        @{user.handle}
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