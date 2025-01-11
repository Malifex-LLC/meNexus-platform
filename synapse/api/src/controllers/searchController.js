// Import Search model
const Search = require('../models/search')

exports.search = async (req, res) => {
    const { query, type } = req.query;

    if (!query || query.trim() === "") {
        return res.status(400).json({ error: 'Search query is required' });
    }

    const results = await Search.search(query, type);
    return res.status(200).json(results);
}