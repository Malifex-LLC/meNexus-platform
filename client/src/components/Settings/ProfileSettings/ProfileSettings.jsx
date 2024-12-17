import "./ProfileSettings.css";
import { useState } from "react";

const ProfileSettings = () => {
    const [profilePicture, setProfilePicture] = useState(null);
    const [username, setUsername] = useState("");
    const [handle, setHandle] = useState("");

    const handleImageChange = (e) => {
        const file = e.target.files[0];
        if (file) {
            setProfilePicture(URL.createObjectURL(file));
        }
    };

    const handleSubmit = (e) => {
        e.preventDefault();
        console.log("Profile Updated:", { username, handle, profilePicture });
    };

    return (
        <div className="profile-settings__container">
            <h2 className="profile-settings__header">Profile Settings</h2>
            <form className="profile-settings__form" onSubmit={handleSubmit}>
                {/* Profile Picture */}
                {profilePicture && (
                    <img
                        src={profilePicture}
                        alt="Profile Preview"
                        className="profile-settings__image-preview"
                    />
                )}
                <label>
                    Profile Picture:
                    <input
                        type="file"
                        accept="image/*"
                        onChange={handleImageChange}
                    />
                </label>

                {/* Username */}
                <label>
                    Username:
                    <input
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                        placeholder="Enter new username"
                    />
                </label>

                {/* Handle */}
                <label>
                    Handle:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)}
                        placeholder="Enter new handle"
                    />
                </label>

                {/* Full Name */}
                <label>
                    Full Name:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)}
                        placeholder="Enter new name"
                    />
                </label>

                {/* Bio */}
                <label>
                    Bio:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)}
                        placeholder="Enter new bio"
                    />
                </label>

                {/* Location */}
                <label>
                    Location:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)}
                        placeholder="Enter new location"
                    />
                </label>

                <button type="submit">Save Changes</button>
            </form>
        </div>
    );
};

export default ProfileSettings;