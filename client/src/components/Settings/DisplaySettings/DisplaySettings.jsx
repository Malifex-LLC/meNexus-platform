import "./DisplaySettings.css";
import { useState } from "react";

const DisplaySettings = () => {
    const [theme, setTheme] = useState("Dark");
    const [fontSize, setFontSize] = useState(16);

    const handleSubmit = (e) => {
        e.preventDefault();
        console.log("Display Settings Updated:", { theme, fontSize });
    };

    return (
        <div className="display-settings__container">
            <h2 className="display-settings__header">Display Settings</h2>
            <form className="display-settings__form" onSubmit={handleSubmit}>
                {/* Theme Selection */}
                <label>
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
                <label>
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
                <button type="submit">Save Changes</button>
            </form>
        </div>
    );
};

export default DisplaySettings;