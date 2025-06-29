// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";

const DisplaySettings = () => {
    const [theme, setTheme] = useState("Dark");
    const [fontSize, setFontSize] = useState(16);

    const handleSubmit = (e) => {
        e.preventDefault();
        console.log("Display Settings Updated:", { theme, fontSize });
    };

    return (
        <div className="display-settings__container flex flex-col p-8 md:mx-16 text-foreground">
            <h2 className="display-settings__header flex text-4xl font-semibold p-8 mb-4 gap-8 items-center rounded-2xl
            bg-surface">Display Settings</h2>
            <form className="display-settings__form flex flex-col p-4" onSubmit={handleSubmit}>
                {/* Theme Selection */}
                <label className={`mb-4`}>
                    Theme:
                    <select
                        value={theme}
                        onChange={(e) => setTheme(e.target.value)}
                    >
                        <option value="Light">Light</option>
                        <option value="Dark">Dark</option>
                    </select>
                </label>

                {/* Font Size Selection */}
                <label className={`mb-4`}>
                    Font Size:
                    <input
                        type="number"
                        min="12"
                        max="24"
                        value={fontSize}
                        onChange={(e) => setFontSize(e.target.value)}
                    />
                </label>

                {/* Submit Button */}
                <button className={`w-xs bg-brand`} type="submit">Save Changes</button>
            </form>
        </div>
    );
};

export default DisplaySettings;