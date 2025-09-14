// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

const SynapseCard = ({
                         name,
                         description,
                         publicKey}) => {
    return (
        <div className='synapseCard p-4 m-4   bg-surface border border-border/30 rounded-xl hover:bg-brand/10 hover:translate-y-[-2px] shadow-lg'>
            <div className="text-sm  text-brand hover:underline font-montserrat">
                {name}
            </div>
            <div className="text-xs text-foreground font-jetbrains">
                {description}
            </div>
            {/*<div className="hidden xl:flex text-xs text-foreground">*/}
            {/*    {publicKey}*/}
            {/*</div>*/}

        </div>
    )
}

export default SynapseCard;