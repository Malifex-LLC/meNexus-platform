import useAxios from './useAxios';

const useGetProfile = () => {
    const { data, loading, error, sendRequest } = useAxios();

    const getProfile = async (handle) => {
        return await sendRequest({ method: 'GET', url: `/api/getProfile/${handle}` });
    };

    return { getProfile, profile: data, loading, error };
};

export default useGetProfile;
