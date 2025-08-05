// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetSynapseMembers = () => {
    console.log("useGetSynapseMembers called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapseMembers = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_MEMBERS,
            withCredentials: true,
        });
        console.log('getSynapseMembers response', response);
        return response.data;
    };

    return {
        getSynapseMembers,
        data,
        loading,
        error};
};

export default useGetSynapseMembers;