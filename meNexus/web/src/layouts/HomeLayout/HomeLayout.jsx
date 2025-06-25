import { useState } from "react";
import Header from "../../components/Header/Header.jsx";
import ActivityFeed from "../../components/Activity/ActivityFeed/ActivityFeed.jsx";
import ControlPanel from "../../components/ControlPanel/ControlPanel.jsx";
import { useSwipeable } from 'react-swipeable';


const HomeLayout = ({ children }) => {
    const [activePanel, setActivePanel] = useState(1); // 0: Social, 1: Main, 2: Activity
    const swipeHandlers = useSwipeable({
        onSwipedLeft: () => setActivePanel((prev) => Math.min(prev + 1, 2)),
        onSwipedRight: () => setActivePanel((prev) => Math.max(prev - 1, 0)),
    });

    return (
        <div className='home-layout h-screen flex flex-col bg-background'>
            <div className='sticky top-0 z-50 border-b border-border'>
                <Header />
                {/* Mobile Nav */}
                <div className='flex lg:hidden justify-around pt-17 py-2 border-b border-border text-foreground'>
                    <button
                        onClick={() => setActivePanel(0)}
                        className={`${activePanel === 0 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Social
                    </button>
                    <button
                        onClick={() => setActivePanel(1)}
                        className={`${activePanel === 1 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Feed
                    </button>
                    <button
                        onClick={() => setActivePanel(2)}
                        className={`${activePanel === 2 ? 'text-[#FF6B6B] font-semibold' :
                            'text-foreground cursor-pointer hover:text-brand'}`}
                    >
                        Activity
                    </button>
                </div>
            </div>



            {/* Content Area */}
            <div className="flex-1 w-full flex flex-col lg:grid lg:grid-cols-12 overflow-hidden"
                {...swipeHandlers}>

                {/* SOCIAL PANEL */}
                <div className={`
                ${activePanel === 0 ? 'flex' : 'hidden'} 
                lg:flex flex-col pt-17 border-r border-border w-full lg:col-span-3`}
                >
                    <ControlPanel />
                </div>

                {/* MAIN CONTENT */}
                <div
                    className={`
                    ${activePanel === 1 ? 'flex' : 'hidden'}
                    lg:flex flex-col pt-17 px-8 overflow-y-auto flex-1 w-full lg:col-span-6`}
                >
                    {children}
                </div>

                {/* ACTIVITY FEED */}
                <div
                    className={`
                    ${activePanel === 2 ? 'flex' : 'hidden'}
                    lg:flex flex-col pt-17 border-l border-border w-full items-center lg:col-span-3`}
                >
                    <ActivityFeed />
                </div>
            </div>

        </div>
    );
};

export default HomeLayout;
