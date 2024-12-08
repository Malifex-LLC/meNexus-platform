import LoginLayout from "../Layouts/LoginLayout.jsx";
import LoginForm from "../LoginForm/LoginForm.jsx";
import React, { useState, useEffect } from 'react';
import axios from "axios";


const LoginPage = () => {

    return(
        <LoginLayout>
            <LoginForm/>
        </LoginLayout>



    );
}

export default LoginPage;
