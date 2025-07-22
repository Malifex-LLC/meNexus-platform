// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetAllDiscoveredPeers = () => {
    console.log("useGetAllDiscoveredPeers called");
    const { data, loading, error, sendRequest } = useAxios();

    const getAllDiscoveredPeers = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_ALL_DISCOVERED_PEERS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getAllDiscoveredPeers,
        data,
        loading,
        error};
};

export default useGetAllDiscoveredPeers;