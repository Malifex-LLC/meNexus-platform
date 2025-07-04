// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import RegisterFormPKI from "../../components/RegisterForm/RegisterForm.jsx";
import RegisterLayout from '../../layouts/RegisterLayout/RegisterLayout.jsx'

const RegisterPage = () => {
    return (
        <RegisterLayout>
            <div className="register__main-content flex flex-col text-xl items-center w-md mb-4 bg-surface text-foreground rounded-xl border border-border shadow-2xl">
                <h1 className={`text-5xl text-center font-bold p-4`}>Welcome to meNexus</h1>
                <p className={`text-xl text-center font-light p-4`}>Create your account to get started!</p>
                <RegisterFormPKI />
            </div>
        </RegisterLayout>
    );
};

export default RegisterPage;