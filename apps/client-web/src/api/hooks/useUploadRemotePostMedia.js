// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useUploadRemotePostMedia = () => {
    const { sendRequest, loading, error } = useAxios();

    const uploadRemotePostMedia = async (file, postId, publicKey, synapsePublicKey) => {
        const formData = new FormData();
        //formData.append('uploadType', 'postMedia');
        formData.append('post_media', file);
        formData.append('postId', postId);
        formData.append('publicKey', publicKey);
        formData.append('synapsePublicKey', synapsePublicKey)

        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.UPLOAD_REMOTE_POST_MEDIA,
                data: formData,
                withCredentials: true
            });
            return response;
        } catch (err) {
            console.error('Error uploading remote post media:', err);
            throw err;
        }
    };

    return {
        uploadRemotePostMedia,
        loading,
        error
    };
};

export default useUploadRemotePostMedia;