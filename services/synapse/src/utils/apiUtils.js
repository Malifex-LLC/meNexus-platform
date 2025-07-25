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
    if (url.includes('youtube.com') || url.includes('youtu.be')) {
        const videoId = extractYouTubeVideoId(url);
        return await fetchYouTubePreview(videoId, process.env.YOUTUBE_API_KEY);
    }

    const { data } = await axios.get(url, {
        timeout: 5000,
        headers: {
            'User-Agent':
                'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36',
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

export function extractYouTubeVideoId(url) {
    const regex = /(?:v=|youtu\.be\/)([0-9A-Za-z_-]{11})/;
    const match = url.match(regex);
    return match ? match[1] : null;
}

export async function fetchYouTubePreview(videoId, apiKey) {
    console.log(`fetchYoutubePreview called for ${videoId}`);
    const res = await axios.get('https://www.googleapis.com/youtube/v3/videos', {
        params: {
            part: 'snippet',
            id: videoId,
            key: apiKey,
        },
    });

    const video = res.data.items[0];
    if (!video) return null;

    const { title, description, thumbnails, channelTitle } = video.snippet;

    return {
        title,
        description,
        image: thumbnails.high?.url || thumbnails.default?.url,
        url: `https://www.youtube.com/watch?v=${videoId}`,
        channel: channelTitle,
    };
}

