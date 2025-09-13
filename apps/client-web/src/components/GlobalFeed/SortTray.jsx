// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const SortTray = () => {
    return (
        <div className={`flex flex-col p-2`}>
            <option value="recent" className={`px-2 hover:bg-brand/50 hover:cursor-pointer rounded-xl`}>Recent</option>
            <option value="chronological" className={`px-2 hover:bg-brand/50 hover:cursor-pointer rounded-xl`}>Chronological</option>
            <option value="trending" className={`px-2 hover:bg-brand/50 hover:cursor-pointer rounded-xl`}>Trending</option>
        </div>
    )
}

export default SortTray;