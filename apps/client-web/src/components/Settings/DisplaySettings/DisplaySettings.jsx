// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useEffect, useState } from "react";
import useThemeLoader  from "../../../api/hooks/useThemeLoader.js"; // Adjust path as needed

const DisplaySettings = () => {
    const [theme, setTheme] = useState(() => localStorage.getItem('theme') || 'SalmonRoasted');
    const [fontSize, setFontSize] = useState(() => parseInt(localStorage.getItem('fontSize')) || 16);

    // Apply theme dynamically
    useThemeLoader(theme);

    const handleSubmit = (e) => {
        e.preventDefault();
        localStorage.setItem('theme', theme);
        localStorage.setItem('fontSize', fontSize);
        console.log("Display Settings Updated:", { theme, fontSize });
    };

    return (
        <div className="flex flex-col  w-full xl:m-4  text-foreground bg-surface/70 border border-border xl:rounded-xl font-montserrat">
            <h2 className="flex text-4xl w-full font-semibold p-8 mb-4 gap-8 items-center  bg-surface rounded-t-xl text-brand border-b border-border">
                Display Settings
            </h2>
            <div className={`flex flex-col xl:grid xl:grid-cols-2 w-full p-4 gap-4`}>
                <form className="flex flex-row col-span-2" onSubmit={handleSubmit}>
                    {/* Theme Selection */}
                    <label className="text-2xl text-brand">
                        Selected Theme:
                        <select
                            className={`text-lg text-foreground px-4`}
                            value={theme}
                            onChange={(e) => setTheme(e.target.value)}
                        >
                            <option value="BlushCloud">BlushCloud</option>
                            <option value="Cappuccino">Cappuccino</option>
                            <option value="Dracula">Dracula</option>
                            <option value="Enki">Enki</option>
                            <option value="GameBoy">GameBoy</option>
                            <option value="GruvboxDark">Gruvbox Dark</option>
                            <option value="HighVisOrange">High Vis - Orange</option>
                            <option value="HighVisYellow">High Vis - Yellow</option>
                            <option value="Indigo">Indigo</option>
                            <option value="Limewire">Limewire</option>
                            <option value="Monokai">Monokai</option>
                            <option value="MoreMonstersAndSprites">More Monsters and Sprites</option>
                            <option value="OneDarkPro">One Dark Pro</option>
                            <option value="RosePine">Rose Pine</option>
                            <option value="SalmonBurnt">Salmon Burnt</option>
                            <option value="SalmonFresh">Salmon Fresh</option>
                            <option value="SalmonRoasted">Salmon Roasted</option>
                            <option value="Sanskrit">Sanskrit</option>
                            <option value="ScaryMonstersAndNiceSprites">Scary Monsters and Nice Sprites</option>
                            <option value="Solarized">Solarized</option>
                            <option value="TurboUnicornConfetti">Turbo Unicorn Confetti</option>
                            <option value="VerdantEmber">Verdant Ember</option>

                            {/* Add more themes here */}
                        </select>
                    </label>
                </form>
                <div className={`col-span-1 grid grid-cols-4 p-4 w-full  rounded-xl`}>
                    <div className={`flex flex-col col-span-2  gap-4 w-full justify-between`}>
                        <div className={`w-full`}>Background Color</div>
                        <div className={`w-full`}>Surface Color</div>
                        <div className={`w-full`}>Brand Color</div>
                        <div className={`w-full`}>Primary Color</div>
                        <div className={`w-full`}>Secondary Color</div>
                        <div className={`w-full`}>Accent Color</div>
                        <div className={`w-full`}>Font Color</div>
                    </div>
                    <div className={`flex flex-col col-span-2 gap-4  w-full `}>
                        <div className={`p-4 w-full bg-background border border-border`}/>
                        <div className={`p-4 w-full bg-surface border border-border`}/>
                        <div className={`p-4 w-full bg-brand border border-border`}/>
                        <div className={`p-4 w-full bg-primary border border-border`}/>
                        <div className={`p-4 w-full bg-secondary border border-border`}/>
                        <div className={`p-4 w-full bg-accent border border-border`}/>
                        <div className={`p-4 w-full bg-foreground border border-border`}/>
                    </div>
                    {/* Submit Button */}
                    <div className={`flex pt-4 xl:pt-16 w-full justify-center items-center col-start-2 col-span-2`}>
                        <button
                            className="m-4 w-full bg-brand hover:bg-brand/70 active:bg-surface cursor-pointer rounded-xl"
                            type="submit"
                            onClick={handleSubmit}
                        >
                            Save Changes
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default DisplaySettings;
