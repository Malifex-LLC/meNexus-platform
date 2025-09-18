const UserInterests = () => {
    const interests = [
        {tag: "Decentralized Tech"},
        {tag: "Programming"},
        {tag: "React"},
        {tag: "C++"}
    ]
    return (
        <div className={'flex flex-col w-full p-4 bg-surface rounded-xl shadow-lg border border-border'}>
            <h1 className={`text-3xl text-brand font-montserrat`}>Interests</h1>
            <div className={'flex w-full flex-wrap gap-2 mt-4 px-2 font-inter'}>
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