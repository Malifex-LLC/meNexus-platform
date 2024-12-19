import "./ProfileSettings.css";
import { useState } from "react";
import useUploadProfilePicture from "../../../api/hooks/useUploadProfilePicture.js";

const ProfileSettings = () => {
    const [username, setUsername] = useState("");
    const [handle, setHandle] = useState("");
    const [selectedFile, setSelectedFile] = useState(null);

    const { uploadProfilePicture, profilePictureloading, profilePictureError } = useUploadProfilePicture();

    const handleFileChange = (event) => {
        setSelectedFile(event.target.files[0]);
    };

    const handleUpload = async () => {
        event.preventDefault();
        if (!selectedFile) {
            console.log('Please select a file to upload.');
            return;
        }

        try {
            const response = await uploadProfilePicture(selectedFile);

            if(response.status === 200) {
                console.log('Profile picture uploaded successfully!');
            }
        } catch (error) {
            console.error('Error uploading profile picture:', error.message);
            console.log('Failed to upload profile picture.');
        }
    };

    // Handle loading and error states for posts
    if (profilePictureloading) {
        return <div>Loading posts...</div>;
    }

    if (profilePictureError) {
        return <div>Error loading posts: {profilePictureError.message}</div>;
    }

    return (
        <div className="profile-settings__container">
            <h2 className="profile-settings__header">Profile Settings</h2>
            <form className="profile-settings__form" onSubmit={handleUpload}>
                {/* Profile Picture */}
                <label>
                    Profile Picture:
                    <input
                        type="file"
                        accept="image/*"
                        onChange={handleFileChange}
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

                <button
                    type="submit"
                    onSubmit={handleUpload}
                >
                    Save Changes
                </button>
            </form>
        </div>
    );
};

export default ProfileSettings;