// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const PostBoard = ({channel, setActiveChannel}) => {
    return (
        <div>
            <button
                className={'w-full text-left cursor-pointer font-inter'}
                onClick={() => setActiveChannel(channel)}
            >
                {channel}
            </button>
        </div>
    )
}

export default PostBoard;