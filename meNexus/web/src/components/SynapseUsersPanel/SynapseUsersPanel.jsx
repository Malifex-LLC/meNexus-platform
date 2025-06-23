import ProfileCard from "../ProfileCard/ProfileCard.jsx";

const SynapseUsersPanel = () => {
    // TODO Implement fetching logic for Synapse Users
    return (
        <div className={'flex flex-col p-4 w-full h-screen text-foreground'}>
            <ProfileCard
                publicKey={"0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf"}
            />
            <ProfileCard
                publicKey={"03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92"}
            />
            <ProfileCard
                publicKey={"0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1"}
            />
            <ProfileCard
                publicKey={"02cc482bc2dbad7c79a9d9f8dbc29f38c61f9134e4f5ee666098ab12f226a3e5de"}
            />
        </div>
    );
}

export default SynapseUsersPanel;