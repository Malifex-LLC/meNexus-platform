// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const FilterTray = () => {
    return (
        <div className={`flex flex-col p-2`}>
            <option value="joined" className={`px-2 hover:bg-brand/50 hover:cursor-pointer rounded-xl`}>Joined Synapses</option>
            <option value="followed" className={`px-2 hover:bg-brand/50 hover:cursor-pointer rounded-xl`}>Followed Users</option>
        </div>
    )
}

export default FilterTray;