import LoginLayout from "../../layouts/LoginLayout.jsx";
import LoginForm from "../../components/LoginForm/LoginForm.jsx";
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
