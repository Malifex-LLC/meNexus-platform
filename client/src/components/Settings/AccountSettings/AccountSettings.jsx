import './AccountSettings.css';

const AccountSettings = () => {
    return (
        <div className="account-settings__container">
            <h2 className="account-settings__header">Account Settings</h2>
            <form className="account-settings__form">
                <label>
                    Email:
                    <input
                        type="email"
                        placeholder="Enter new email"
                    />
                </label>
                <label>
                    Password:
                    <input
                        type="password"
                        placeholder="Enter new password"
                    />
                </label>
                <label>
                    Confirm Password:
                    <input
                        type="password"
                        placeholder="Confirm new password"
                    />
                </label>
                <button type="submit">
                    Save Changes
                </button>
            </form>
        </div>
    );
};

export default AccountSettings;