import useAxios from './useAxios';
import useGetProfile from "./useGetProfile.jsx";

const useGetUserPosts = () => {
    console.log("useGetUserPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getUserPosts = async (handle) => {
        return await sendRequest({method: 'GET', url: `/api/getUserPosts/${handle}` });
    };

    return {getUserPosts, posts: data, loading, error};
};

export default useGetUserPosts;