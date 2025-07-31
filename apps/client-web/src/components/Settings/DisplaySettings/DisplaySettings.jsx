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
        <div className="display-settings__container flex flex-col p-8 md:mx-16 text-foreground">
            <h2 className="display-settings__header flex text-4xl font-semibold p-8 mb-4 gap-8 items-center rounded-2xl bg-surface">
                Display Settings
            </h2>
            <form className="display-settings__form flex flex-col p-4" onSubmit={handleSubmit}>
                {/* Theme Selection */}
                <label className="mb-4">
                    Theme:
                    <select
                        value={theme}
                        onChange={(e) => setTheme(e.target.value)}
                    >
                        <option value="BlushCloud">BlushCloud</option>
                        <option value="Cappuccino">Cappuccino</option>
                        <option value="Dracula">Dracula</option>
                        <option value="Enki">Enki</option>
                        <option value="GameBoy">GameBoy</option>
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

                {/* Submit Button */}
                <button
                    className="mt-8 w-md bg-brand hover:bg-primary active:bg-surface cursor-pointer"
                    type="submit"
                    onClick={handleSubmit}
                >
                    Save Changes
                </button>
            </form>
        </div>
    );
};

export default DisplaySettings;
