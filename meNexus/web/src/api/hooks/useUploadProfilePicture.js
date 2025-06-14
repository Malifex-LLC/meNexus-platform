import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useUploadProfilePicture = () => {
    const { sendRequest, loading, error } = useAxios();

    const uploadProfilePicture = async (file) => {
        const formData = new FormData();
        formData.append('uploadType', 'profilePicture');
        formData.append('profile_picture', file);

        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.UPLOAD_PROFILE_PICTURE,
                data: formData,
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
                withCredentials: true
            });
            return response;
        } catch (err) {
            console.error('Error uploading profile picture:', err);
            throw err;
        }
    };

    return { uploadProfilePicture, loading, error };
};

export default useUploadProfilePicture;