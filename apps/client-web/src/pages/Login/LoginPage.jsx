// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import LoginLayoutPKI from "../../layouts/LoginLayout/LoginLayoutPKI.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";

const LoginPage = () => {
    return(
        <LoginLayoutPKI>
            <div className="register__main-content p-4 w-md bg-surface/70 text-forground text-center text-xl rounded-xl border border-border shadow-2xl">
                <h1 className={'text-foreground text-5xl text-center font-bold p-4'}>Welcome to meNexus</h1>
                <p className={'text-foreground text-xl text-center font-light p-4'}>Login to your account to get started!</p>
                <LoginForm/>
            </div>
        </LoginLayoutPKI>
    );
}

export default LoginPage;