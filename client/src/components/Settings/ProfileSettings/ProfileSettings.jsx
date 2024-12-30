import "./ProfileSettings.css";
import {useEffect, useState} from "react";
import useUploadProfilePicture from "../../../api/hooks/useUploadProfilePicture.js";
import useGetProfile from "../../../api/hooks/useGetProfile.js";
import {useNavigate} from "react-router-dom";
import useGetSessionUser from "../../../api/hooks/useGetSessionUser.js";
import useUpdateProfileSettings from "../../../api/hooks/useUpdateProfileSettings.js";

const ProfileSettings = () => {
    const [sessionUserId, setsessionUserId] = useState(null);
    const [sessionUserHandle, setsessionUserHandle] = useState(null);
    const [newHandle, setNewHandle] = useState(null);
    const [sessionUserDisplayName, setSessionUserDisplayName] = useState(null);
    const [newDisplayName, setNewDisplayName] = useState(null);
    const [sessionUserProfileName, setSessionUserProfileName] = useState(null);
    const [newProfileName, setNewProfileName] = useState(null);
    const [sessionUserProfileBio, setSessionUserProfileBio] = useState(null);
    const [newProfileBio, setNewProfileBio] = useState(null);
    const [sessionUserProfileLocation, setSessionUserProfileLocation] = useState(null);
    const [newProfileLocation, setNewProfileLocation] = useState(null);
    const [isHandleSet, setIsHandleSet] = useState(false);
    const [profile, setProfile] = useState({});
    const [selectedFile, setSelectedFile] = useState(null);
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getProfile, loading: profileLoading, error: profileError } = useGetProfile();
    const { uploadProfilePicture, profilePictureloading, profilePictureError } = useUploadProfilePicture();
    const { updateProfileSettings, profileSettingsUpdateLoading, profileSettingsUpdateError } = useUpdateProfileSettings();

    const handleProfileUpdate = async () => {
        event.preventDefault(); // Prevent form submission reload

        const updatedFields = {};

        if (newDisplayName && newDisplayName !== sessionUserDisplayName) {
            updatedFields.display_name = newDisplayName;
        }
        if (newHandle && newHandle !== sessionUserHandle) {
            updatedFields.handle = newHandle;
        }
        if (newProfileName && newProfileName !== sessionUserProfileName) {
            updatedFields.profile_name = newProfileName;
        }
        if (newProfileBio && newProfileBio !== sessionUserProfileBio) {
            updatedFields.profile_bio = newProfileBio;
        }
        if (newProfileLocation && newProfileLocation !== sessionUserProfileLocation) {
            updatedFields.profile_location = newProfileLocation;
        }

        // If no fields have been updated, return early
        if (Object.keys(updatedFields).length === 0) {
            console.log("No changes to update.");
            return;
        }

        const response = updateProfileSettings(sessionUserHandle, updatedFields);
        if (response.status === 200) {
            console.log("Profile updated successfully");
        }

    }

    const handleFileChange = (event) => {
        setSelectedFile(event.target.files[0]);
    };

    const handleProfilePictureUpload = async () => {
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

    // Fetch Session User's user_id and handle
    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!sessionUserHandle && !isHandleSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();

                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setIsHandleSet(true);
                        setsessionUserId(response.data.user_id);
                        setsessionUserHandle(response.data.handle);
                    } else {
                        console.error("Invalid session, redirecting to login.");
                        navigate('/login');
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                    navigate('/login');
                }
            } else if (sessionUserHandle) {
                const response = await getSessionUser();
                setsessionUserId(response.data.user_id);
                setsessionUserHandle(response.data.handle);
                setIsHandleSet(true);
            }
        };

        fetchSessionUser();
    }, [sessionUserHandle, navigate, isHandleSet]);

    // Fetch profile and posts once the current handle is determined
    useEffect(() => {
        if (sessionUserHandle && isHandleSet) {
            const fetchData = async () => {
                try {
                    console.log("Fetching profile and posts for handle:", sessionUserHandle);
                    const [profileData] = await Promise.all([
                        getProfile(sessionUserHandle),
                    ]);
                    console.log("Profile Data after getProfile() fetching is:", profileData);
                    setProfile(profileData);
                    setSessionUserDisplayName(profileData.display_name);
                    setSessionUserProfileName(profileData.profile_name);
                    setSessionUserProfileBio(profileData.profile_bio);
                    setSessionUserProfileLocation(profileData.profile_location);
                } catch (error) {
                    console.error("Error fetching data:", error);
                }
            };

            fetchData();
        }
    }, [sessionUserHandle, isHandleSet]);

    return (
        <div className="profile-settings__container">
            <h2 className="profile-settings__header">Profile Settings</h2>
            <form className="profile-settings__form">
                {/* Profile Picture */}
                <label>
                    Profile Picture:
                    <input
                        type="file"
                        accept="image/*"
                        onChange={handleFileChange}
                    />
                </label>
                <button
                    type="button"
                    onClick={handleProfilePictureUpload}
                >
                    Upload
                </button>

                {/* Username */}
                <label>
                    Display Name:
                    <input
                        type="text"
                        value={newDisplayName}
                        onChange={(e) => setNewDisplayName(e.target.value)}
                        placeholder={sessionUserDisplayName}
                    />
                </label>

                {/* Handle */}
                <label>
                    Handle:
                    <input
                        type="text"
                        value={newHandle}
                        onChange={(e) => setNewHandle(e.target.value)}
                        placeholder={sessionUserHandle}
                    />
                </label>

                {/* Full Name */}
                <label>
                    Profile Name:
                    <input
                        type="text"
                        value={newProfileName}
                        onChange={(e) => setNewProfileName(e.target.value)}
                        placeholder={sessionUserProfileName}
                    />
                </label>

                {/* Bio */}
                <label>
                    Bio:
                    <input
                        type="text"
                        value={newProfileBio}
                        onChange={(e) => setNewProfileBio(e.target.value)}
                        placeholder="Enter new bio"
                    />
                </label>

                {/* Location */}
                <label>
                    Location:
                    <input
                        type="text"
                        value={newProfileLocation}
                        onChange={(e) => setNewProfileLocation(e.target.value)}
                        placeholder="Enter new location"
                    />
                </label>

                <button
                    type="button"
                    onClick={handleProfileUpdate}
                >
                    Save Changes
                </button>
            </form>
        </div>
    );
};

export default ProfileSettings;