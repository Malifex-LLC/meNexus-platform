// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import axios from 'axios';
import * as cheerio from 'cheerio';

export async function sendRequest({method, url, data ={}, params ={}, withCredentials = true}) {
    return axios({
        method: method,
        url: url,
        data: data,
        params: params,
        withCredentials: withCredentials,
    });
}

export async function fetchLinkPreview(url) {
    const { data } = await axios.get(url, {
        timeout: 5000,
        headers: {
            'User-Agent':
                'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36',
            'Accept-Language': 'en-US,en;q=0.9',
        },
    });

    const $ = cheerio.load(data);

    const getMeta = (name) =>
        $(`meta[property="${name}"]`).attr('content') ||
        $(`meta[name="${name}"]`).attr('content');

    return {
        title: getMeta('og:title') || $('title').text(),
        description: getMeta('og:description'),
        image: getMeta('og:image'),
        url: getMeta('og:url') || url,
    };
}
