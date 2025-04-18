import {useState} from "react";
import useUpdateAccountSettings from '../../../api/hooks/useUpdateAccountSettings.js'
import {FaKey} from "react-icons/fa";

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
        } else if (response.status === 400 || response.status === 500) {
            setIsUpdateSuccess(false);
            setIsUpdateError(true);
        }
    }

    return (
        <div className="account-settings__container flex flex-col p-8 mx-16 text-foreground">
            <div className="account-settings__header flex text-4xl font-semibold p-8 mb-4 gap-8 items-center
            bg-surface">
                <FaKey />
                Account Settings
            </div>
            <form className="account-settings__form flex flex-col p-4">
                <label className={`flex flex-col w-md mb-4`}>
                    Email:
                    <input
                        className={`border border-border p-2`}
                        type="email"
                        value={newEmail}
                        onChange={(e) => setNewEmail(e.target.value)}
                        placeholder="Enter new email"
                    />
                </label>
                <label className={`flex flex-col w-md mb-4`}>
                    Password:
                    <input
                        className={`border border-border p-2`}
                        type="password"
                        value={newPassword}
                        onChange={(e) => setNewPassword(e.target.value)}
                        placeholder="Enter new password"
                    />
                </label>
                <label className={`flex flex-col w-md mb-4`}>
                    Confirm Password:
                    <input
                        className={`border border-border p-2`}
                        type="password"
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                        placeholder="Confirm new password"
                    />
                </label>
                <button
                    className={` w-md bg-brand`}
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