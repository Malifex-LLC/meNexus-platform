// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useUploadPostMedia = () => {
    const { sendRequest, loading, error } = useAxios();

    const uploadPostMedia = async (file, postId, publicKey) => {
        const formData = new FormData();
        //formData.append('uploadType', 'postMedia');
        formData.append('post_media', file);
        formData.append('postId', postId);
        formData.append('publicKey', publicKey);

        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.UPLOAD_POST_MEDIA,
                data: formData,
                withCredentials: true
            });
            return response;
        } catch (err) {
            console.error('Error uploading post media:', err);
            throw err;
        }
    };

    return {
        uploadPostMedia,
        loading,
        error
    };
};

export default useUploadPostMedia;