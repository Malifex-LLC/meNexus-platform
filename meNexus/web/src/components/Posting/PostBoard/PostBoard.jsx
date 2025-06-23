const PostBoard = ({channel, setActiveChannel}) => {
    return (
        <div>
            <button
                className={'w-full text-left cursor-pointer'}
                onClick={() => setActiveChannel(channel)}
            >
                {channel}
            </button>
        </div>
    )
}

export default PostBoard;