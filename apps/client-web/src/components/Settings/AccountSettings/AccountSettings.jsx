// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

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
        <div className="flex flex-col  w-full xl:m-4  text-foreground bg-surface/70 border border-border xl:rounded-xl font-montserrat">
            <div className="flex text-4xl w-full font-semibold p-8 mb-4 gap-8 items-center  bg-surface rounded-t-xl text-brand border-b border-border">
                <FaKey />
                Account Settings
            </div>
            <div className={`xl:grid xl:grid-cols-4`}>
                <form className="col-span-2 flex flex-col p-4 ">
                    <label className={`flex flex-col w-full mb-4`}>
                        Email:
                        <input
                            className={`border border-border p-2 focus:outline-1 focus:outline-brand/60`}
                            type="email"
                            value={newEmail}
                            onChange={(e) => setNewEmail(e.target.value)}
                            placeholder="Enter new email"
                        />
                    </label>
                    <label className={`flex flex-col w-full mb-4`}>
                        Password:
                        <input
                            className={`border border-border p-2 focus:outline-1 focus:outline-brand/60`}
                            type="password"
                            value={newPassword}
                            onChange={(e) => setNewPassword(e.target.value)}
                            placeholder="Enter new password"
                        />
                    </label>
                    <label className={`flex flex-col w-full mb-4`}>
                        Confirm Password:
                        <input
                            className={`border border-border p-2 focus:outline-1 focus:outline-brand/60`}
                            type="password"
                            value={confirmPassword}
                            onChange={(e) => setConfirmPassword(e.target.value)}
                            placeholder="Confirm new password"
                        />
                    </label>
                    <button
                        className={` w-full bg-brand rounded-xl`}
                        type="button"
                        onClick={handleAccountUpdate}
                    >
                        Save Changes
                    </button>
                </form>
            </div>
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