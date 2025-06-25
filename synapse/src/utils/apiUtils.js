// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import axios from 'axios';

export async function sendRequest({method, url, data ={}, params ={}, withCredentials = true}) {
    return axios({
        method: method,
        url: url,
        data: data,
        params: params,
        withCredentials: withCredentials,
    });
}