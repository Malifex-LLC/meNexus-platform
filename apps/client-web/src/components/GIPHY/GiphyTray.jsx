import {useEffect, useState} from "react";
import useGiphySearch from "../../api/hooks/useGiphySearch.js";
import useCreatePost from "../../api/hooks/useCreatePost.js";
import GiphyImage from "./GiphyImage.jsx"

const GiphyTray = ({isLocalSynapse, publicKey, activeChannel, sendMessage, toggleGiphyTray}) => {
    const [formClicked, setFormClicked] = useState(false);
    const [text, setText] = useState('Search GIPHY');
    const [url, setUrl] = useState('');
    const [images, setImages] = useState();

    const { giphySearch } = useGiphySearch();
    const { createPost } = useCreatePost();


    const handleFormClick = () => {
        if (!formClicked && text === `Search GIPHY`) {
            setText("");
        }
        setFormClicked(true);
    };

    const handleSubmit = async () => {
        if (!text.trim()) return;
        const gifResults = await giphySearch(text);
        setImages(gifResults);

    };


    return (
        <div className={`flex flex-col w-full h-full bg-surface/70 backdrop-blur-xs rounded-t-xl`}>
            <div className={`sticky top-0 bg-surface border-b border-border`}>
                <div className={`text-center text-brand text-2xl font-montserrat font-semibold`}>GIPHY Search</div>
                <div className={`w-full p-2`} onClick={handleFormClick}>
                <textarea
                    className="w-full h-full bg-background text-foreground rounded-2xl p-2 focus:outline-1 focus:outline-brand/60"
                    rows={1}
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault(); // Prevent newline
                            handleSubmit();
                        }
                    }}
                />
                </div>
            </div>
            {images && images.length > 0 && (
                <div className={`flex flex-row flex-wrap gap-2 w-full h-full overflow-y-auto p-2`}>
                        {images.map((image, index) => (
                            <GiphyImage
                                key = {index}
                                image={image}
                                publicKey={publicKey}
                                activeChannel={activeChannel}
                                sendMessage={sendMessage}
                                toggleGiphyTray={toggleGiphyTray}
                            />
                        ))}
                </div>
            )}
        </div>
    )
}

export default GiphyTray