// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const Channel = ({channel, setActiveChannel}) => {
    return (
        <div className={''}>
            <button
                className={'w-full text-left  cursor-pointer'}
                onClick={() => setActiveChannel(channel)}
            >
                {channel}
            </button>
        </div>
    )
}

export default Channel;