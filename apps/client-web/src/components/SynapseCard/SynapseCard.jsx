// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const SynapseCard = ({
                         name,
                         description,
                         publicKey}) => {
    return (
        <div className='synapseCard p-4 m-4 max-w-3xl  bg-surface border border-border/30 rounded-xl
        hover:bg-brand/10 hover:translate-y-[-2px] shadow-lg'>
            <div className="text-2xl text-brand hover:underline">
                {name}
            </div>
            <div className="text-lg text-foreground">
                {description}
            </div>
            <div className="text-xs text-foreground">
                {publicKey}
            </div>

        </div>
    )
}

export default SynapseCard;