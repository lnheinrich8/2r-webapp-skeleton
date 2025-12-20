import { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import axios from 'axios';

// import { useAuth } from '../context/AuthContext';
// import { hybridEncrypt, loadPublicKeyFromPemUrl } from '../utils/EncryptPayload';
import API_BASE_URL from '../shared/utils/api';

import './login.css';

const Login = () => {
    // const { setUser, checkAuth } = useAuth();

    const navigate = useNavigate();
    const location = useLocation();

    const registrationSuccess = location.state?.registrationSuccess;
    const passwordChanged = location.state?.passwordChanged;
    const [fadeOut, setFadeOut] = useState(false); // for success popup

    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');

    // const [publicKey, setPublicKey] = useState(null);

    const handleLogin = async (e) => {
        e.preventDefault();
        try {
            // const key = publicKey || await loadPublicKeyFromPemUrl('/keys/public_key.pem');
            // setPublicKey(key);
            // const encryptedPayload = await hybridEncrypt(key, { email, password });

            const res = await axios.post(
                `${API_BASE_URL}/auth/login`,
                { email, password }
                // { withCredentials: true }
            );

            setUser(res.data.user);
            navigate('/dashboard');
            checkAuth();
        } catch (err) {
            const serverError = err.response?.data?.error || "Unknown error occurred during login";
            setError(`Login failed: ${serverError}`);
        }
    };

    const handleRegisterRedirect = (e) => {
        e.preventDefault();
        navigate('/register');
    };

    useEffect(() => {
        if (registrationSuccess || passwordChanged) {
            const fadeTimer = setTimeout(() => setFadeOut(true), 4000);
            const removeTimer = setTimeout(() => {
                navigate(location.pathname, { replace: true, state: {} });
            }, 5000);

            return () => {
                clearTimeout(fadeTimer);
                clearTimeout(removeTimer);
            };
        }
    }, [registrationSuccess, passwordChanged, location.pathname, navigate]);

    return (
        <div className="login-page">
            {registrationSuccess && (
                <div className={`popup-success ${fadeOut ? 'fade-out' : ''}`}>
                    Click the verification link in your email to finish registration
                </div>
            )}
            {passwordChanged && (
                <div className={`popup-success ${fadeOut ? 'fade-out' : ''}`}>
                    Password changed successfully. Please log in with the new password
                </div>
            )}
            <form onSubmit={handleLogin} className="login-form">
                <h2>Welcome Back</h2>
                {error && <p className="message-error">{error}</p>}
                <input
                    type="email"
                    placeholder="Email"
                    value={email}
                    className="general-input-1"
                    onChange={e => setEmail(e.target.value)}
                    required
                />
                <input
                    type="password"
                    placeholder="Password"
                    value={password}
                    className="general-input-1"
                    onChange={e => setPassword(e.target.value)}
                    required
                />
                <button className="general-button-1" type="submit">Login</button>

                <p className="register-text">
                    Don’t have an account?{' '}
                    <span className="register-link" onClick={handleRegisterRedirect}>
                        Register
                    </span>
                </p>

            </form>
        </div>
    );
};

export default Login;
