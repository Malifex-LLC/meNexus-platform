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