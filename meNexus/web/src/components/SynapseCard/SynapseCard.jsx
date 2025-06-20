const SynapseCard = ({
                         name,
                         description,
                         publicKey}) => {
    return (
        <div className='synapseCard p-4 m-4 max-w-3xl border'>
            <div className="text-5xl text-brand">
                {name}
            </div>
            <div className="text-3xl text-foreground">
                {description}
            </div>
            <div className="text-md text-foreground">
                publicKey: {publicKey}
            </div>
        </div>
    )
}

export default SynapseCard;