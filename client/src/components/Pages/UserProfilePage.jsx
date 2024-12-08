// UserProfilePage.jsx
import React from "react";
import UserProfile from "../UserProfile/UserProfile.jsx";
import UserProfileLayout from "../Layouts/UserProfileLayout.jsx";

const UserProfilePage = () => {
    return (
        <UserProfileLayout>
            <UserProfile />
        </UserProfileLayout>
    );
};

export default UserProfilePage;
