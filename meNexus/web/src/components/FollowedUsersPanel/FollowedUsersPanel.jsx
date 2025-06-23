import ProfileCard from "../ProfileCard/ProfileCard.jsx";
import { useEffect, useState } from "react";
import useGetUser from "../../api/hooks/useGetUser.js";

const FollowedUsersPanel = ({following}) => {
    const { getUser } = useGetUser();

    const [allUsers, setAllUsers] = useState([])

    useEffect(() => {
        const fetchAllUsers = async () => {
            const usersList = await Promise.all(
                following.map(async (user) => {
                    return await getUser(user);
                })
            )
            setAllUsers(usersList)
        }
        fetchAllUsers();
    }, [])

    return (
        <div className={'bg-background text-foreground p-4 m-4 text-center shadow-2xl'}>
            <div className={'text-3xl '}>
                Followed Users
            </div>
            <div className={'p-4 m-4 text-left'}>
                {allUsers.map((user) => (
                    <ProfileCard
                        publicKey={user.publicKey}
                        handle={user.handle}
                        profilePicture={user.profilePicture}
                    />
                ))}
            </div>
        </div>
    );
}

export default FollowedUsersPanel