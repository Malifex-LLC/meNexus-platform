// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import LoginLayoutPKI from "../../layouts/LoginLayout/LoginLayoutPKI.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";
import {Link} from "react-router-dom";

const LoginPage = () => {
    return(
        <LoginLayoutPKI>
            <div className="register__main-content p-4 max-w-md bg-surface/70 text-forground text-center text-xl rounded-xl border border-border shadow-2xl">
                <h1 className={'text-foreground text-5xl text-center font-bold p-4'}>Welcome to <span className={`font-oxanium text-brand`}>meNexus</span></h1>
                <p className={'text-foreground text-xl text-center font-light p-4'}>Login to your account to get started!</p>
                <LoginForm/>
                <div className='login-layout__register-redirect text-xl text-foreground mt-4'>
                    Don't have an account? <Link to="/register" className={`text-brand`}>Register!</Link>
                </div>
            </div>
        </LoginLayoutPKI>
    );
}

export default LoginPage;