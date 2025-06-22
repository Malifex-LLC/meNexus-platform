const SynapseCard = ({
                         name,
                         description,
                         publicKey}) => {
    return (
        <div className='synapseCard p-4 m-4 max-w-3xl border bg-surface rounded-xl'>
            <div className="text-2xl text-brand">
                {name}
            </div>
            <div className="text-lg text-foreground">
                {description}
            </div>

        </div>
    )
}

export default SynapseCard;