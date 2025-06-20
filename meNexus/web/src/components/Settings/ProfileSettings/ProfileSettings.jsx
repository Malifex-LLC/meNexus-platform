import {useEffect, useState} from "react";
import useUploadProfilePicture from "../../../api/hooks/useUploadProfilePicture.js";
import useGetProfile from "../../../api/hooks/useGetProfile.js";
import {useNavigate} from "react-router-dom";
import useGetSessionUser from "../../../api/hooks/useGetSessionUser.js";
import useUpdateProfileSettings from "../../../api/hooks/useUpdateProfileSettings.js";
import useGetUserByHandle from "../../../api/hooks/useGetUserByHandle.js";

const ProfileSettings = () => {
    const [user, setUser] = useState(null)
    const [sessionPublicKey, setsessionPublicKey] = useState(null);
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
    const [selectedFile, setSelectedFile] = useState(null);
    const [ isUpdateSuccess, setIsUpdateSuccess ] = useState(false);
    const [ isUpdateError, setIsUpdateError ] = useState(false);
    const [ isUploadSuccess, setIsUploadSuccess ] = useState(false);
    const [ isUploadError, setIsUploadError ] = useState(false);
    const [previewUrl, setPreviewUrl] = useState(null);
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getUserByHandle, loading: userLoading, error: userError } = useGetUserByHandle();
    const { uploadProfilePicture, profilePictureloading, profilePictureError } = useUploadProfilePicture();
    const { updateProfileSettings, profileSettingsUpdateLoading, profileSettingsUpdateError } = useUpdateProfileSettings();

    const handleProfileUpdate = async () => {
        event.preventDefault(); // Prevent form submission reload
        setIsUpdateError(false);

        const updatedFields = {};

        if (newDisplayName && newDisplayName !== sessionUserDisplayName) {
            updatedFields.displayName = newDisplayName;
        }
        if (newHandle && newHandle !== sessionUserHandle) {
            updatedFields.handle = newHandle;
        }
        if (newProfileName && newProfileName !== sessionUserProfileName) {
            updatedFields.profileName = newProfileName;
        }
        if (newProfileBio && newProfileBio !== sessionUserProfileBio) {
            updatedFields.bio = newProfileBio;
        }
        if (newProfileLocation && newProfileLocation !== sessionUserProfileLocation) {
            updatedFields.location = newProfileLocation;
        }

        // If no fields have been updated, return early
        if (Object.keys(updatedFields).length === 0) {
            console.log("No changes to update.");
            return;
        }

        const response = await updateProfileSettings(sessionPublicKey, updatedFields);
        if (response.status === 200) {
            console.log("Profile settings updated successfully");
            setIsUpdateSuccess(true);
            setIsUpdateError(false);
            setNewDisplayName("");
            setNewHandle("");
            setNewProfileName("");
            setNewProfileBio("");
            setNewProfileBio("");
        } else if (response.status === 401) {
            setIsUpdateSuccess(false);
            setIsUpdateError(true);
        }

    }

    const handleFileChange = (event) => {
        const file = event.target.files[0];
        setSelectedFile(file);

        if (file) {
            const preview = URL.createObjectURL(file);
            setPreviewUrl(preview);
            console.log('Preview URL: ', preview);
        } else {
            setPreviewUrl(null);
        }
    };


    const handleProfilePictureUpload = async () => {
        event.preventDefault();
        setIsUploadError(false);
        setIsUploadSuccess(false);

        if (!selectedFile) {
            console.log('Please select a file to upload.');
            return;
        }

        try {
            const response = await uploadProfilePicture(selectedFile);

            if(response.status === 200) {
                console.log('Profile picture uploaded successfully!');
                setIsUploadSuccess(true);
            }
        } catch (error) {
            console.error('Error uploading profile picture:', error.message);
            console.log('Failed to upload profile picture.');
            setIsUploadError(true);
        }
    };

    // Fetch Session User's user_id and handle
    useEffect(() => {
        const fetchSessionUser = async () => {
            if (!sessionUserHandle && !isHandleSet) {
                try {
                    console.log("Fetching current user session...");
                    const response = await getSessionUser();
                    console.log('sessionUser response ', response);
                    const userData = await getUserByHandle(response.data.handle);
                    setUser(userData);
                    console.log('userData response: ', userData);
                    if (response.status === 200 && response.data.handle) {
                        console.log("Session user handle:", response.data.handle);
                        setIsHandleSet(true);
                        setsessionPublicKey(response.data.publicKey);
                        setsessionUserHandle(response.data.handle);
                        setSessionUserDisplayName(userData.displayName);
                        setSessionUserProfileName(userData.profileName);
                        setSessionUserProfileBio(userData.bio);
                        setSessionUserProfileLocation(userData.location);
                    } else {
                        console.error("Invalid session, redirecting to login.");
                        navigate('/login');
                    }
                } catch (error) {
                    console.error("Error fetching current user session:", error);
                    navigate('/login');
                }
            }
        };

        fetchSessionUser();
    }, [sessionUserHandle, navigate, isHandleSet, user]);

    return (
        <div className="profile-settings__container flex-1  p-8 h-full  md:mx-16 text-foreground">
            <h2 className="profile-settings__header flex text-4xl font-semibold p-8 mb-4 gap-8 rounded-2xl
            bg-surface">Profile Settings</h2>
            <div className={`flex flex-col xl:flex-row gap-8 p-4 `}>
                <div className={``}>
                    {/* Username */}
                    <label className={`flex flex-col  w-md mb-4`}>
                        Display Name:
                        <input
                            className={`border border-border p-2`}
                            type="text"
                            value={newDisplayName}
                            onChange={(e) => setNewDisplayName(e.target.value)}
                            placeholder={sessionUserDisplayName}
                        />
                    </label>

                    {/* Handle */}
                    <label className={`flex flex-col w-md mb-4`}>
                        Handle:
                        <input
                            className={`border border-border p-2`}
                            type="text"
                            value={newHandle}
                            onChange={(e) => setNewHandle(e.target.value)}
                            placeholder={sessionUserHandle}
                        />
                    </label>

                    {/* Full Name */}
                    <label className={`flex flex-col w-md mb-4`}>
                        Profile Name:
                        <input
                            className={`border border-border p-2`}
                            type="text"
                            value={newProfileName}
                            onChange={(e) => setNewProfileName(e.target.value)}
                            placeholder={sessionUserProfileName}
                        />
                    </label>

                    {/* Bio */}
                    <label className={`flex flex-col w-md mb-4`}>
                        Bio:
                        <input
                            className={`border border-border p-2`}
                            type="text"
                            value={newProfileBio}
                            onChange={(e) => setNewProfileBio(e.target.value)}
                            placeholder="Enter new bio"
                        />
                    </label>

                    {/* Location */}
                    <label className={`flex flex-col w-md mb-4`}>
                        Location:
                        <input
                            className={`border border-border p-2`}
                            type="text"
                            value={newProfileLocation}
                            onChange={(e) => setNewProfileLocation(e.target.value)}
                            placeholder="Enter new location"
                        />
                    </label>

                    <button
                        className={`mt-8 w-md bg-brand hover:bg-primary active:bg-surface`}
                        type="button"
                        onClick={handleProfileUpdate}
                    >
                        Save Changes
                    </button>
                    {isUpdateSuccess && (
                        <p className="profile-settings__updated-successfully text-brand">
                            Profile settings updated successfully
                        </p>
                    )}
                    {isUpdateError && (
                        <p className="profile-settings__updated-error text-brand">
                            Profile settings failed to update
                        </p>
                    )}
                </div>
                <form className="profile-settings__form flex-1 flex flex-col overflow-y-auto">
                    {/* Profile Picture */}
                    <div className="flex flex-col mb-6">
                        <span className="mb-2 text-lg font-semibold">Profile Picture</span>
                        <label htmlFor="profilePicture" className={`cursor-pointer flex items-center justify-center 
                    w-32 h-12 bg-brand hover:bg-primary active:bg-surface text-white rounded-xl 
                    border border-border transition-colors duration-150`}
                        >
                            Choose File
                        </label>

                        <input
                            id="profilePicture"
                            type="file"
                            accept="image/*"
                            onChange={handleFileChange}
                            className="hidden"
                        />

                        {selectedFile && (
                            <span className="mt-2 text-sm text-foreground">
                            Selected: {selectedFile.name}
                        </span>
                        )}
                    </div>
                    {previewUrl && (
                        <img
                            src={previewUrl}
                            alt="Profile Preview"
                            className="mt-4 w-64 self-center mb-8  border border-border"
                        />
                    )}
                    <button
                        className={`w-md bg-brand hover:bg-primary active:bg-surface`}
                        type="button"
                        onClick={handleProfilePictureUpload}
                    >
                        Upload
                    </button>
                    {isUploadSuccess && (
                        <p className="profile-settings__updated-successfully text-brand">
                            Profile picture uploaded successfully!
                        </p>
                    )}
                    {isUploadError && (
                        <p className="profile-settings__updated-error text-brand">
                            Profile picture failed to upload. Please try again.
                        </p>
                    )}
                </form>
            </div>
        </div>
    );
};

export default ProfileSettings;