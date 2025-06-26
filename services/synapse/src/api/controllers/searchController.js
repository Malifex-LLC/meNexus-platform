// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Search model
import Search from '../models/search.js'

export const search = async (req, res) => {
    const { query, type } = req.query;

    if (!query || query.trim() === "") {
        return res.status(400).json({ error: 'Search query is required' });
    }

    const results = await Search.search(query, type);
    return res.status(200).json(results);
}

export default { search };