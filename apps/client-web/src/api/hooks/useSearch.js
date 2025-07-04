// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";
import { replaceParams } from "../../utils/apiUtils.js";



const useSearch = () => {
    const { data, loading, error, sendRequest } = useAxios();

    const search = async (params) => {
        console.log("Search for query: ", params.query);
        const url = replaceParams(ENDPOINTS.SEARCH, {params});

        const response = await sendRequest( {
            method: "GET",
            url: url,
            params: params,
        });
        console.log("useSearch response: ", response);
        return response.data;

    }
    return {
        search,
        searchResults: data,
        loading,
        error
    }
}

export default useSearch;