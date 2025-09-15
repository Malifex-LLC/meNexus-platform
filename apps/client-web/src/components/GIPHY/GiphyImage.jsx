import { useState } from "react";

const GiphyImage = ({image, publicKey,activeChannel, sendMessage, toggleGiphyTray}) => {
    const previewUurl = image.images.fixed_height_small.url
    const fullSizeUrl = image.images.fixed_height.url


    const handleSendGif = () => {
        sendMessage({
            type: 'chatMessage',
            publicKey: publicKey,
            activeChannel: activeChannel,
            content: fullSizeUrl
        });
        toggleGiphyTray()
    }

    return (
        <div className={`border  border-border hover:border-4 hover:border-brand/60 hover:cursor-pointer`}>
            <button onClick={handleSendGif}>
                <img src={`${previewUurl}`} alt={`awesome gif`}/>
            </button>
        </div>
    )
}

export default GiphyImage