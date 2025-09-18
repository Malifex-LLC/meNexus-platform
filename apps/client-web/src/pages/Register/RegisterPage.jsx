// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import RegisterFormPKI from "../../components/RegisterForm/RegisterForm.jsx";
import RegisterLayout from '../../layouts/RegisterLayout/RegisterLayout.jsx'
import {Link} from "react-router-dom";

const RegisterPage = () => {
    return (
        <RegisterLayout>
            <div className="flex flex-col text-xl items-center w-full max-w-md  p-4 bg-surface/70 text-foreground rounded-xl border border-border shadow-2xl">
                <h1 className={`text-5xl text-center font-bold p-4`}>Welcome to <span className={`font-oxanium text-brand`}>meNexus</span></h1>
                <p className={`text-xl text-center font-light p-4`}>Create your account to get started!</p>
                <RegisterFormPKI />
                <div className='register-layout__login-redirect text-xl text-foreground mt-4'>
                    Already have an account? <Link to="/login" className={'text-brand'}>Login!</Link>
                </div>
            </div>
        </RegisterLayout>
    );
};

export default RegisterPage;