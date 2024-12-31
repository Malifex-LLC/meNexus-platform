import './AccountSettings.css';
import {useState} from "react";
import useUpdateAccountSettings from '../../../api/hooks/useUpdateAccountSettings.js'

const AccountSettings = () => {
    const [ newEmail, setNewEmail ] = useState(null);
    const [ newPassword, setNewPassword ] = useState(null);
    const [ confirmPassword, setConfirmPassword ] = useState(null);
    const [ isPasswordMatch, setIsPasswordMatch ] = useState(true);
    const [ isUpdateSuccess, setIsUpdateSuccess ] = useState(false);
    const [ isUpdateError, setIsUpdateError ] = useState(false);

    const { updateAccountSettings, accountSettingsUpdateLoading, accountSettingsUpdateError } = useUpdateAccountSettings();

    const handleAccountUpdate = async () => {
        event.preventDefault();
        setIsUpdateError(false);

        const updatedFields = {};

        if (newEmail) {
            updatedFields.email = newEmail;
        }

        if (newPassword) {
            if(newPassword === confirmPassword) {
                updatedFields.password = newPassword;
                setIsPasswordMatch(true);
                setIsUpdateSuccess(true);
            } else {
                setIsPasswordMatch(false);
                setIsUpdateError(true);
            }
        }

        const response = await updateAccountSettings(updatedFields);

        if (response.status === 200) {
            console.log("Account settings updated successfully");
            setIsUpdateSuccess(true);
            setIsUpdateError(false);
            setNewPassword("");
            setConfirmPassword("");
            setNewEmail("");
        } else if (response.status === 401) {
            setIsUpdateSuccess(false);
            setIsUpdateError(true);
        }
    }

    return (
        <div className="account-settings__container">
            <h2 className="account-settings__header">Account Settings</h2>
            <form className="account-settings__form">
                <label>
                    Email:
                    <input
                        type="email"
                        value={newEmail}
                        onChange={(e) => setNewEmail(e.target.value)}
                        placeholder="Enter new email"
                    />
                </label>
                <label>
                    Password:
                    <input
                        type="password"
                        value={newPassword}
                        onChange={(e) => setNewPassword(e.target.value)}
                        placeholder="Enter new password"
                    />
                </label>
                <label>
                    Confirm Password:
                    <input
                        type="password"
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                        placeholder="Confirm new password"
                    />
                </label>
                <button
                    type="button"
                    onClick={handleAccountUpdate}
                >
                    Save Changes
                </button>
            </form>
            <p className="account-settings__passwords-dont-match">
                {!isPasswordMatch && "Passwords Don't Match"}
            </p>
            {isUpdateSuccess && (
                <p className="account-settings__updated-successfully">
                    Account settings updated successfully
                </p>
            )}
            {isUpdateError && (
                <p className="account-settings__updated-error">
                    Account settings failed to update
                </p>
            )}
        </div>
    );
};

export default AccountSettings;