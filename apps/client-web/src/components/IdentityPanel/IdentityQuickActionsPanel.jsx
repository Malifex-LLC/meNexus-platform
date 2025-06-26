// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const IdentityQuickActionsPanel = () => {
    return (
        <div className={'flex gap-8   text-foreground'}>
            <button
                className={`p-2 w-32 bg-surface rounded-xl cursor-pointer shadow-lg hover:bg-primary hover:tranlsate-y-[-2px]`}
                onClick={() => {}}
            >
                Follow
            </button>
            <button
                className={`p-2 w-32 bg-surface rounded-xl cursor-pointer shadow-lg hover:bg-primary hover:tranlsate-y-[-2px]`}
                onClick={() => {}}
            >
                DM
            </button>
            <button
                className={`p-2 w-32 bg-surface rounded-xl cursor-pointer shadow-lg hover:bg-primary hover:tranlsate-y-[-2px]`}
                onClick={() => {}}
            >
                Invite
            </button>
        </div>
    )
}

export default IdentityQuickActionsPanel;