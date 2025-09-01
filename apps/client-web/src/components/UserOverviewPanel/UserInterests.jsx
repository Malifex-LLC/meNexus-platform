const UserInterests = () => {
    const interests = [
        {tag: "Decentralized Tech"},
        {tag: "Programming"},
        {tag: "React"},
        {tag: "C++"}
    ]
    return (
        <div className={'flex flex-col p-4 bg-surface rounded-xl shadow-lg border border-border'}>
            <h1 className={`text-3xl text-brand`}>Interests</h1>
            <div className={'flex gap-2 mt-4 px-2'}>
                {interests.map((interest, index) => (
                    <span
                        key={index}
                        className={`bg-primary/30 text-neutral rounded-xl px-2`}
                    >
                        {interest.tag}
                    </span>
                ))}
            </div>
        </div>
    )
}

export default UserInterests;