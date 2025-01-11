import useAxios from './useAxios.js';
import { ENDPOINTS } from '../../config/endpoints.js';
import { replaceParams } from "../../utils/apiUtils.js";

const useUpdateProfileSettings = () => {
    const { sendRequest, loading, error } = useAxios();

    const updateProfileSettings = async (handle, updatedFields) => {
        const url = replaceParams(ENDPOINTS.UPDATE_PROFILE_SETTINGS, {handle});
        console.log('updateProfileSettings called for fields: ', updatedFields);

        try {
            const response = await sendRequest({
                method: 'PUT',
                url: url,
                data: updatedFields,
                withCredentials: true
        });
            return response;

        } catch (error) {
            console.error('Error updating profile settings: ', error);
            throw error;
        }
    };

    return {
        updateProfileSettings,
        loading,
        error
    };
};

export default useUpdateProfileSettings;