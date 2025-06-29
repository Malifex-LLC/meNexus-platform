// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';
import { replaceParams } from "../../utils/apiUtils.js";

const useFollowActions = () => {
    const { sendRequest, loading, error } = useAxios();

    const followUser = async (followedPublicKey) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.FOLLOW_USER,
                data: {followedPublicKey},
                withCredentials: true,
            });

            return response.data;
        } catch (err) {
            console.error('Error following user: ', err);
            throw err;
        }
    };

    const unfollowUser = async (followedPublicKey) => {
        try {
            const url = replaceParams(ENDPOINTS.UNFOLLOW_USER, {followedPublicKey});

            const response = await sendRequest({
                method: 'DELETE',
                url: url,
                data: {followedPublicKey},
                withCredentials: true,
            });
            return response.data;
        } catch (err) {
            console.error('Error unfollowing user:', err);
            throw err;
        }
    };

    const followCheck = async (followedPublicKey) => {
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FOLLOW_CHECK,
            params: {followedPublicKey},
            withCredentials: true,
        });
        return response.data.isFollowing;
    };

    return { followUser, unfollowUser, followCheck, loading, error };
};

export default useFollowActions;